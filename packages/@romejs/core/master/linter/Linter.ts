/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {MasterRequest} from '@romejs/core';
import CompilerLinter from './CompilerLinter';
import {LINTABLE_EXTENSIONS} from '@romejs/core/common/fileHandlers';
import {parsePathPattern, PathPatterns} from '@romejs/path-match';
import {AbsoluteFilePathSet} from '@romejs/path';
import DependencyGraph from '../dependencies/DependencyGraph';
import {humanizeNumber} from '@romejs/string-utils';

const globalIgnorePatterns: PathPatterns = [
  parsePathPattern({input: 'node_modules'}),
  parsePathPattern({input: '__generated__'}),
];

function concatLintIgnore(patterns: PathPatterns): PathPatterns {
  // Only add the global ignore patterns when there's no ignore negate patterns
  // When there's a negate with a single segments, all files are basically included since they'll all
  // "not match" the global ones with the same priority
  for (const pattern of patterns) {
    if (pattern.negate) {
      return patterns;
    }
  }

  return patterns.concat(globalIgnorePatterns);
}

export default class Linter {
  constructor(req: MasterRequest) {
    this.request = req;
  }

  request: MasterRequest;

  static async getFilesFromArgs(
    request: MasterRequest,
    extenstions: Array<string>,
  ): Promise<AbsoluteFilePathSet> {
    return request.getFilesFromArgs(
      project => concatLintIgnore(project.config.lint.ignore),
      extenstions,
    );
  }

  async lint() {
    const {request} = this;
    const {reporter} = request;

    const printer = request.createDiagnosticsPrinter({
      category: 'lint',
      message: 'Dispatched',
    });

    const paths: AbsoluteFilePathSet = await Linter.getFilesFromArgs(
      request,
      LINTABLE_EXTENSIONS,
    );

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

    throw printer;
  }
}
