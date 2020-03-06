/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {MasterRequest} from '@romejs/core';
import {DiagnosticsPrinter} from '@romejs/cli-diagnostics';
import {ProjectDefinition} from '@romejs/project';
import {AbsoluteFilePathSet} from '@romejs/path';

export default class CompilerLinter {
  constructor(req: MasterRequest, printer: DiagnosticsPrinter) {
    this.request = req;
    this.printer = printer;
  }

  request: MasterRequest;
  printer: DiagnosticsPrinter;

  async lint(paths: AbsoluteFilePathSet) {
    const {request, printer} = this;
    const {master, reporter} = request;

    const pathsByWorker = await master.fileAllocator.groupPathsByWorker(paths);

    const spinner = reporter.progress();
    spinner.setTitle('Linting');
    spinner.setTotal(paths.size);

    const lintDisabledProjects: Set<ProjectDefinition> = new Set();

    await Promise.all(
      pathsByWorker.map(async paths => {
        for (const path of paths) {
          spinner.setText(`<filelink target="${path.join()}" />`);

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
                    "Files excluded from linting as it's not enabled for this project. Run `rome config enable-category lint` to enable it.",
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
          const {
            diagnostics,
            suppressions,
          } = await this.request.requestWorkerLint(path, false);
          printer.processor.addSuppressions(suppressions);
          printer.addDiagnostics(diagnostics);

          spinner.tick();
        }

        spinner.setText('Done');
        spinner.pause();
      }),
    );

    spinner.end();
  }
}
