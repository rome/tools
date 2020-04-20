/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Worker, FileReference} from '@romejs/core';
import {Program} from '@romejs/js-ast';
import {Diagnostics, descriptions, catchDiagnostics} from '@romejs/diagnostics';
import {
  TransformStageName,
  CompileResult,
  CompilerOptions,
  compile,
} from '@romejs/js-compiler';
import {
  WorkerParseOptions,
  WorkerCompilerOptions,
  WorkerFormatResult,
  WorkerLintResult,
  WorkerLintOptions,
} from '../common/bridges/WorkerBridge';
import Logger from '../common/utils/Logger';
import * as jsAnalysis from '@romejs/js-analysis';
import {
  getFileHandlerAssert,
  ExtensionLintResult,
} from '../common/fileHandlers';
import {
  AnalyzeDependencyResult,
  UNKNOWN_ANALYZE_DEPENDENCIES_RESULT,
} from '../common/types/analyzeDependencies';
import {matchPathPatterns} from '@romejs/path-match';

// Some Windows git repos will automatically convert Unix line endings to Windows
// This retains the line endings for the formatted code if they were present in the source
function normalizeFormattedLineEndings(
  sourceText: string,
  formatted: string,
): string {
  if (sourceText.includes('\r')) {
    return formatted.replace(/\n/g, '\r\n');
  } else {
    return formatted;
  }
}

export default class WorkerAPI {
  constructor(worker: Worker) {
    this.worker = worker;
    this.logger = worker.logger;
  }

  worker: Worker;
  logger: Logger;

  interceptAndAddGeneratedToDiagnostics<T extends {diagnostics: Diagnostics}>(
    val: T,
    generated: boolean,
  ): T {
    if (generated) {
      const diagnostics = val.diagnostics.map(
        (diag) => {
          const diagAdvice = diag.description.advice === undefined
            ? []
            : diag.description.advice;
          return {
              ...diag,
              metadata: {
                ...diag.description,
                advice: [
                  ...diagAdvice,
                  {
                    type: 'log',
                    category: 'warn',
                    message: 'This diagnostic was generated on a file that has been converted to JavaScript. The source locations are most likely incorrect',
                  },
                ],
              },
            };
        },
      );

      return {...val, diagnostics};
    } else {
      return val;
    }
  }

  async moduleSignatureJS(ref: FileReference, parseOptions: WorkerParseOptions) {
    const {ast, project} = await this.worker.parseJS(ref, parseOptions);

    this.logger.info(`Generating export types:`, ref.real);

    return await jsAnalysis.getModuleSignature({
      ast,
      project,
      provider: await this.worker.getTypeCheckProvider(
        ref.project,
        {},
        parseOptions,
      ),
    });
  }

  async analyzeDependencies(
    ref: FileReference,
    parseOptions: WorkerParseOptions,
  ): Promise<AnalyzeDependencyResult> {
    const project = this.worker.getProject(ref.project);
    const {handler} = getFileHandlerAssert(ref.real, project.config);
    this.logger.info(`Analyze dependencies:`, ref.real);

    const {analyzeDependencies} = handler;
    if (analyzeDependencies === undefined) {
      return UNKNOWN_ANALYZE_DEPENDENCIES_RESULT;
    }

    return analyzeDependencies({
      file: ref,
      project,
      worker: this.worker,
      parseOptions,
    });
  }

  async workerCompilerOptionsToCompilerOptions(
    ref: FileReference,
    workerOptions: WorkerCompilerOptions,
    parseOptions: WorkerParseOptions,
  ): Promise<CompilerOptions> {
    const {bundle, ...options} = workerOptions;

    if (bundle === undefined) {
      return options;
    } else {
      return {
        ...options,
        bundle: {
          ...bundle,
          analyze: await this.analyzeDependencies(ref, parseOptions),
        },
      };
    }
  }

  async compileJS(
    ref: FileReference,
    stage: TransformStageName,
    options: WorkerCompilerOptions,
    parseOptions: WorkerParseOptions,
  ): Promise<CompileResult> {
    const {ast, project, sourceText, generated} = await this.worker.parseJS(
      ref,
      parseOptions,
    );
    this.logger.info(`Compiling:`, ref.real);

    const compilerOptions = await this.workerCompilerOptionsToCompilerOptions(
      ref,
      options,
      parseOptions,
    );
    return this.interceptAndAddGeneratedToDiagnostics(await compile({
      ast,
      sourceText,
      options: compilerOptions,
      project,
      stage,
    }), generated);
  }

  async parseJS(ref: FileReference, opts: WorkerParseOptions): Promise<Program> {
    let {ast, generated} = await this.worker.parseJS(ref, {
      ...opts,
      sourceType: opts.sourceType,
      cache: false,
    });

    return this.interceptAndAddGeneratedToDiagnostics(ast, generated);
  }

  async format(
    ref: FileReference,
    opts: WorkerParseOptions,
  ): Promise<undefined | WorkerFormatResult> {
    const res = await this._format(ref, opts);
    if (res === undefined) {
      return undefined;
    } else {
      return {
        formatted: normalizeFormattedLineEndings(res.sourceText, res.formatted),
        original: res.sourceText,
        diagnostics: res.diagnostics,
      };
    }
  }

  shouldFormat(ref: FileReference): boolean {
    const project = this.worker.getProject(ref.project);

    return project.config.format.enabled && matchPathPatterns(
        ref.real,
        project.config.lint.ignore,
      ) === 'NO_MATCH' &&
        matchPathPatterns(ref.real, project.config.format.ignore) ===
        'NO_MATCH';
  }

  async _format(
    ref: FileReference,
    parseOptions: WorkerParseOptions,
  ): Promise<undefined | ExtensionLintResult> {
    const project = this.worker.getProject(ref.project);
    this.logger.info(`Formatting:`, ref.real);

    if (!this.shouldFormat(ref)) {
      return;
    }

    const {handler} = getFileHandlerAssert(ref.real, project.config);
    const {format} = handler;
    if (format === undefined) {
      return;
    }

    const res = await format({
      file: ref,
      project,
      worker: this.worker,
      parseOptions,
    });

    return res;
  }

  async lint(
    ref: FileReference,
    options: WorkerLintOptions,
    parseOptions: WorkerParseOptions,
  ): Promise<WorkerLintResult> {
    const project = this.worker.getProject(ref.project);
    this.logger.info(`Linting:`, ref.real);

    // Get the extension handler
    const {handler} = getFileHandlerAssert(ref.real, project.config);

    const {lint} = handler;
    if (lint === undefined && handler.format === undefined) {
      return {
        fixed: false,
        diagnostics: [],
        suppressions: [],
      };
    }

    // Catch any diagnostics, in the case of syntax errors etc
    const res = await catchDiagnostics(() => {
      if (lint === undefined) {
        return this._format(ref, parseOptions);
      } else {
        return lint({
          format: this.shouldFormat(ref),
          file: ref,
          project,
          worker: this.worker,
          options,
          parseOptions,
        });
      }
    }, {
      category: 'lint',
      message: 'Caught by WorkerAPI.lint',
    });

    // These are fatal diagnostics
    if (res.diagnostics !== undefined) {
      return {
        fixed: false,
        suppressions: [],
        diagnostics: res.diagnostics,
      };
    }

    // `format` could have return undefined
    if (res.value === undefined) {
      return {
        fixed: false,
        diagnostics: [],
        suppressions: [],
      };
    }

    // These are normal diagnostics returned from the linter
    const {
      sourceText,
      diagnostics,
      suppressions,
    }: ExtensionLintResult = res.value;

    const formatted = normalizeFormattedLineEndings(
      sourceText,
      res.value.formatted,
    );

    // If the file has pending fixes
    const needsFix = formatted !== sourceText;

    // Autofix if necessary
    if (options.fix && needsFix) {
      // Save the file and evict it from the cache
      await this.worker.writeFile(ref.real, formatted);

      // Relint this file without fixing it, we do this to prevent false positive error messages
      return {
        ...(await this.lint(ref, {...options, fix: false}, parseOptions)),
        fixed: true,
      };
    }

    // If there's no pending fix then no need for diagnostics
    if (!needsFix) {
      return {
        fixed: false,
        diagnostics,
        suppressions,
      };
    }

    // Add pending autofix diagnostic
    return {
      fixed: false,
      suppressions,
      diagnostics: [
        ...diagnostics,
        {
          location: {
            filename: ref.uid,
          },
          description: descriptions.LINT.PENDING_FIXES(sourceText, formatted),
        },
      ],
    };
  }
}
