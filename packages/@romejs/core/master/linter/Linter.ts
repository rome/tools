/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {MasterRequest} from '@romejs/core';
import CompilerLinter from './CompilerLinter';
import {LINTABLE_EXTENSIONS} from '@romejs/core/common/fileHandlers';
import DependencyGraph from '../dependencies/DependencyGraph';
import {DiagnosticLocation, descriptions} from '@romejs/diagnostics';

export type LinterOptions = {
  fixLocation?: DiagnosticLocation;
  args?: Array<string>;
};

export default class Linter {
  constructor(req: MasterRequest, opts: LinterOptions) {
    this.request = req;
    this.options = opts;
  }

  request: MasterRequest;
  options: LinterOptions;

  async lint(throwAlways: boolean = true) {
    const {request, options} = this;
    const {reporter} = request;

    const printer = request.createDiagnosticsPrinter({
      category: 'lint',
      message: 'Dispatched',
    });

    printer.processor.addAllowedUnusedSuppressionPrefix('bundler');

    const {paths, projects} = await request.getFilesFromArgs({
      args: options.args,
      noun: 'lint',
      verb: 'linting',
      configCategory: 'lint',
      extensions: LINTABLE_EXTENSIONS,
      disabledDiagnosticCategory: 'lint/disabled',
    });

    const {fixLocation} = options;
    const shouldFix = fixLocation !== undefined;

    if (fixLocation !== undefined) {
      for (const project of projects) {
        if (!project.config.format.enabled) {
          printer.addDiagnostic({
            location: fixLocation,
            description: descriptions.FORMAT.DISABLED,
          });
        }
      }
    }

    printer.onBeforeFooterPrint((reporter, isError) => {
      if (isError) {
        let couldFix = false;
        let hasPendingFixes = false;

        for (const {
          description: metadata,
        } of printer.processor.getDiagnostics()) {
          if (metadata.category === 'lint/pendingFixes') {
            hasPendingFixes = true;
          }

          if (metadata.fixable) {
            couldFix = true;
          }
        }

        if (hasPendingFixes) {
          reporter.info(
            'Fixes available. Run <command>rome lint --fix</command> to apply.',
          );
        } else if (couldFix) {
          reporter.warn(
            'Autofixes are available for some of these errors when formatting is enabled. Run <command>rome config enable-category format</command> to enable.',
          );
        }
      } else {
        const fileCount = paths.size;
        if (fileCount === 0) {
          reporter.warn('No files linted');
        } else if (fileCount === 1) {
          reporter.info(`<emphasis>1</emphasis> file linted`);
        } else {
          reporter.info(`<number emphasis>${fileCount}</number> files linted`);
        }
      }
    });

    // Add a filter so that only files that are explicitly referenced will be included

    // For example, we don't want to show analysis or parse errors for transitive dependencies if the user only requested a specific file
    printer.processor.addFilter({
      test: (diag) => {
        const {
          location: {filename},
        } = diag;
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
        callback: async () => {
          const compilerLinter = new CompilerLinter(request, printer, shouldFix);
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
