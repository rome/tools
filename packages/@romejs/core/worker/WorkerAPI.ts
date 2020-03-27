/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Worker, FileReference} from '@romejs/core';
import {Program, program} from '@romejs/js-ast';
import {Diagnostics, descriptions, catchDiagnostics} from '@romejs/diagnostics';
import {
  TransformStageName,
  CompileResult,
  CompilerOptions,
  compile,
} from '@romejs/js-compiler';
import {
  PrefetchedModuleSignatures,
  WorkerParseOptions,
  WorkerCompilerOptions,
  WorkerFormatResult,
  WorkerLintResult,
} from '../common/bridges/WorkerBridge';
import Logger from '../common/utils/Logger';
import {removeLoc} from '@romejs/js-ast-utils';
import * as jsAnalysis from '@romejs/js-analysis';
import {getFileHandlerAssert, ExtensionLintResult} from '../common/fileHandlers';
import {
  AnalyzeDependencyResult,
  UNKNOWN_ANALYZE_DEPENDENCIES_RESULT,
} from '../common/types/analyzeDependencies';
import {matchPathPatterns} from '@romejs/path-match';

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
      const diagnostics = val.diagnostics.map((diag) => {
        const diagAdvice = diag.description.advice === undefined
          ? [] : diag.description.advice;
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
      });

      return {...val, diagnostics};
    } else {
      return val;
    }
  }

  async moduleSignatureJS(ref: FileReference) {
    const {ast, project} = await this.worker.parseJS(ref);

    this.logger.info(`Generating export types:`, ref.real);

    return await jsAnalysis.getModuleSignature({
      ast,
      project,
      provider: await this.worker.getTypeCheckProvider(ref.project),
    });
  }

  async analyzeDependencies(ref: FileReference): Promise<AnalyzeDependencyResult> {
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
    });
  }

  async workerCompilerOptionsToCompilerOptions(
    ref: FileReference,
    workerOptions: WorkerCompilerOptions,
  ): Promise<CompilerOptions> {
    const {bundle, ...options} = workerOptions;

    if (bundle === undefined) {
      return options;
    } else {
      return {
        ...options,
        bundle: {
          ...bundle,
          analyze: await this.analyzeDependencies(ref),
        },
      };
    }
  }

  async compileJS(
    ref: FileReference,
    stage: TransformStageName,
    workerOptions: WorkerCompilerOptions,
  ): Promise<CompileResult> {
    const {ast, project, sourceText, generated} = await this.worker.parseJS(ref);
    this.logger.info(`Compiling:`, ref.real);

    const options = await this.workerCompilerOptionsToCompilerOptions(
      ref,
      workerOptions,
    );
    return this.interceptAndAddGeneratedToDiagnostics(await compile({
      ast,
      sourceText,
      options,
      project,
      stage,
    }), generated);
  }

  async parseJS(ref: FileReference, opts: WorkerParseOptions): Promise<Program> {
    let {ast, generated} = await this.worker.parseJS(ref, {
      sourceType: opts.sourceType,
      cache: false,
    });

    ast = this.interceptAndAddGeneratedToDiagnostics(ast, generated);

    if (opts.compact) {
      return program.assert(removeLoc(ast));
    } else {
      return ast;
    }
  }

  async format(ref: FileReference): Promise<undefined | WorkerFormatResult> {
    const res = await this._format(ref);
    if (res === undefined) {
      return undefined;
    } else {
      return {
        formatted: res.formatted,
        original: res.sourceText,
        diagnostics: res.diagnostics,
      };
    }
  }

  shouldFormat(ref: FileReference): boolean {
    const project = this.worker.getProject(ref.project);

    return project.config.format.enabled && matchPathPatterns(
      ref.real,
      project.config.format.ignore,
    ) === 'NO_MATCH';
  }

  async _format(ref: FileReference): Promise<undefined | ExtensionLintResult> {
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
    });

    return res;
  }

  async lint(
    ref: FileReference,
    prefetchedModuleSignatures: PrefetchedModuleSignatures,
    fix: boolean,
  ): Promise<WorkerLintResult> {
    const project = this.worker.getProject(ref.project);
    this.logger.info(`Linting:`, ref.real);

    // Get the extension handler
    const {handler} = getFileHandlerAssert(ref.real, project.config);

    const {lint} = handler;
    if (lint === undefined && handler.format === undefined) {
      return {
        diagnostics: [],
        suppressions: [],
      };
    }

    // Catch any diagnostics, in the case of syntax errors etc
    const res = await catchDiagnostics({
      category: 'lint',
      message: 'Caught by WorkerAPI.lint',
    }, () => {
      if (lint === undefined) {
        return this._format(ref);
      } else {
        return lint({
          format: this.shouldFormat(ref),
          file: ref,
          project,
          prefetchedModuleSignatures,
          worker: this.worker,
        });
      }
    });

    // These are fatal diagnostics
    if (res.diagnostics !== undefined) {
      return {
        suppressions: [],
        diagnostics: res.diagnostics,
      };
    }

    // `format` could have return undefined
    if (res.value === undefined) {
      return {
        diagnostics: [],
        suppressions: [],
      };
    }

    // These are normal diagnostics returned from the linter
    const {
      formatted,
      sourceText: raw,
      diagnostics,
      suppressions,
    }: ExtensionLintResult = res.value;

    // If the file has pending fixes
    const needsFix = formatted !== raw;

    // Autofix if necessary
    if (fix && needsFix) {
      // Save the file and evict it from the cache
      await this.worker.writeFile(ref.real, formatted);

      // Relint this file without fixing it, we do this to prevent false positive error messages
      return this.lint(ref, prefetchedModuleSignatures, false);
    }

    // If there's no pending fix then no need for diagnostics
    if (!needsFix) {
      return {
        diagnostics,
        suppressions,
      };
    }

    // Add pending autofix diagnostic
    return {
      suppressions,
      diagnostics: [
        ...diagnostics,
        {
          location: {
            filename: ref.uid,
          },
          description: descriptions.LINT.PENDING_FIXES(raw, formatted),
        },
      ],
    };
  }
}
