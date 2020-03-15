/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {MasterRequest} from '@romejs/core';
import CompilerLinter from './CompilerLinter';
import {LINTABLE_EXTENSIONS} from '@romejs/core/common/fileHandlers';
import {AbsoluteFilePathSet} from '@romejs/path';
import DependencyGraph from '../dependencies/DependencyGraph';
import {humanizeNumber} from '@romejs/string-utils';

export default class Linter {
  constructor(req: MasterRequest) {
    this.request = req;
  }

  request: MasterRequest;

  async lint(throwAlways: boolean = true) {
    const {request} = this;
    const {reporter, master} = request;

    const printer = request.createDiagnosticsPrinter({
      category: 'lint',
      message: 'Dispatched',
    });

    printer.processor.addAllowedUnusedSuppressionPrefix('bundler');

    const paths: AbsoluteFilePathSet = await request.getFilesFromArgs({
      getProjectIgnore: project => ({
        patterns: project.config.lint.ignore,
        source: master.projectManager.findProjectConfigConsumer(
          project,
          consumer =>
            consumer.has('lint') && consumer.get('lint').has('ignore')
              ? consumer.get('lint').get('ignore')
              : undefined,
        ),
      }),
      getProjectEnabled: project => ({
        enabled: project.config.lint.enabled,
        source: master.projectManager.findProjectConfigConsumer(
          project,
          consumer =>
            consumer.has('lint')
              ? consumer.get('lint').get('enabled')
              : undefined,
        ),
      }),
      noun: 'lint',
      verb: 'linting',
      configCategory: 'lint',
      extensions: LINTABLE_EXTENSIONS,
    });

    printer.onBeforeFooterPrint(() => {
      const fileCount = paths.size;
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

    // Add a filter so that only files that are explicitly referenced will be included
    // For example, we don't want to show analysis or parse errors for transitive dependencies if the user only requested a specific file
    printer.processor.addFilter({
      test: diag => {
        const {filename} = diag;
        if (filename === undefined) {
          return false;
        }

        const path = request.master.projectManager.getFilePathFromUid(filename);
        if (path === undefined) {
          return false;
        }

        return !paths.has(path);
      },
    });

    await reporter.steps([
      {
        clear: true,
        message: 'Analyzing files',
        async callback() {
          const compilerLinter = new CompilerLinter(request, printer);
          await compilerLinter.lint(paths);
        },
      },
      {
        clear: true,
        message: 'Analyzing dependencies',
        async callback() {
          const analyzeProgress = reporter.progress();
          analyzeProgress.setTitle('Analyzing');

          const graph = new DependencyGraph(request, {});
          await graph.seed({
            paths: Array.from(paths),
            diagnosticsProcessor: printer.processor,
            validate: false,
            analyzeProgress,
          });

          for (const path of paths) {
            graph.validate(graph.getNode(path), printer.processor);
          }
        },
      },
    ]);

    if (throwAlways || printer.hasDiagnostics()) {
      throw printer;
    } else {
      printer.footer();
    }
  }
}
