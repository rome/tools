/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {MasterRequest} from '@romejs/core';
import {DiagnosticsPrinter} from '@romejs/cli-diagnostics';
import {PartialDiagnostics} from '@romejs/diagnostics';
import {humanizeNumber} from '@romejs/string-utils';
import {parsePathPattern} from '@romejs/path-match';
import {ProjectDefinition} from '@romejs/project';

export default class CompilerLinter {
  constructor(req: MasterRequest, printer: DiagnosticsPrinter) {
    this.request = req;
    this.printer = printer;
  }

  request: MasterRequest;
  printer: DiagnosticsPrinter;

  async lint() {
    const {request, printer} = this;
    const {master, reporter} = request;
    const globalIgnore = [
      parsePathPattern({input: 'node_modules'}),
      parsePathPattern({input: '__generated__'}),
    ];

    const files = await request.getFilesFromArgs(project =>
      project.config.lint.ignore.concat(globalIgnore),
    );

    const filesByWorker = await master.fileAllocator.groupFilesByWorker(files);

    const spinners = filesByWorker.map(files => {
      const spinner = reporter.progress();
      spinner.setTotal(files.length);
      return spinner;
    });

    const lintDisabledProjects: Set<ProjectDefinition> = new Set();

    const startTime = Date.now();
    let totalProcessedBytes = 0;

    await Promise.all(
      filesByWorker.map(async (files, workerNum) => {
        const spinner = spinners[workerNum];

        let i = 0;
        for (const path of files) {
          i++;
          spinner.setCurrent(i);
          spinner.setText(path.join());
          totalProcessedBytes += master.memoryFs.getFileStatsAssert(path).size;

          // Complain about the project config if it has lint disabled
          const project = master.projectManager.findProjectExisting(path);
          if (project !== undefined && !project.config.lint.enabled) {
            if (!lintDisabledProjects.has(project)) {
              lintDisabledProjects.add(project);

              const {
                consumer,
                value,
              } = master.projectManager.findProjectConfigConsumer(
                project,
                consumer =>
                  consumer.has('lint')
                    ? consumer.get('lint').get('enabled')
                    : undefined,
              );

              if (value === undefined) {
                printer.addDiagnostic({
                  category: '',
                  message:
                    'Files excluded from linting as it\'s not enabled. Add `lint: {"enabled": true}`',
                  ...consumer.getDiagnosticPointer(),
                });
              } else {
                printer.addDiagnostic({
                  category: '',
                  message:
                    "Files excluded from linting as it's disabled in this project config",
                  ...value.getDiagnosticPointer('value'),
                });
              }
            }
            continue;
          }

          // TODO support `fix` flag
          const fileDiagnostics: PartialDiagnostics = await this.request.requestWorkerLint(
            path,
            false,
          );
          printer.addDiagnostics(fileDiagnostics);

          spinner.tick();
        }

        spinner.setText('Done');
        spinner.pause();
      }),
    );

    for (const spinner of spinners) {
      spinner.end();
    }

    const elapsed = Date.now() - startTime;

    printer.onBeforeFooterPrint(reporter => {
      const kbps = Math.round(totalProcessedBytes / elapsed);
      reporter.verbose(`Processing speed ${humanizeNumber(kbps)}kbps`);

      const fileCount = files.length;
      if (fileCount === 0) {
        reporter.warn('No files linted');
      } else if (fileCount === 1) {
        reporter.info(`<emphasis>1</emphasis> file linted`);
      } else {
        reporter.info(
          `<emphasis>${humanizeNumber(fileCount)}</emphasis> files linted`,
        );
      }
    });
  }
}
