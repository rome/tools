/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {PartialDiagnostics, DiagnosticSuppressions} from '@romejs/diagnostics';
import {TransformRequest} from '../types';
import {lintTransforms} from '../transforms/lint/index';
import {program} from '@romejs/js-ast';
import {Cache, Context} from '@romejs/js-compiler';
import {generateJS} from '@romejs/js-generator';
import {extractSuppressionsFromProgram} from '../suppressions';

export type LintResult = {
  diagnostics: PartialDiagnostics;
  suppressions: DiagnosticSuppressions;
  src: string;
};

const lintCache: Cache<LintResult> = new Cache();

export default async function lint(req: TransformRequest): Promise<LintResult> {
  const {ast, sourceText, project} = req;

  if (!project.config.lint.enabled) {
    return {
      diagnostics: [],
      suppressions: extractSuppressionsFromProgram(ast),
      src: sourceText,
    };
  }

  const query = Cache.buildQuery(req);
  const cached = lintCache.get(query);
  if (cached) {
    return cached;
  }

  let formattedCode = sourceText;
  if (project.config.format.enabled) {
    // Perform autofixes
    const context = new Context({
      ast,
      project,
      origin: {
        category: 'lint',
      },
    });
    const newAst = program.assert(
      context.reduce(ast, lintTransforms, {frozen: false}),
    );

    const generator = generateJS(
      newAst,
      {
        typeAnnotations: true,
      },
      sourceText,
    );
    formattedCode = generator.getCode() + '\n';
  }

  // Run lints (could be with the autofixed AST)
  const context = new Context({
    ast,
    project,
    origin: {
      category: 'lint',
    },
  });
  program.assert(context.reduce(ast, lintTransforms, {frozen: true}));

  const result: LintResult = {
    suppressions: extractSuppressionsFromProgram(ast),
    diagnostics: [...ast.diagnostics, ...context.diagnostics],
    src: formattedCode,
  };
  lintCache.set(query, result);
  return result;
}
