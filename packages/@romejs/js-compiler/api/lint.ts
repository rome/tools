/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Diagnostics, DiagnosticSuppressions} from '@romejs/diagnostics';
import {TransformRequest} from '../types';
import {lintTransforms} from '../transforms/lint/index';
import {program} from '@romejs/js-ast';
import {Cache, Context} from '@romejs/js-compiler';
import {generateJS} from '@romejs/js-generator';

export type LintResult = {
  diagnostics: Diagnostics;
  suppressions: DiagnosticSuppressions;
  src: string;
};

const lintCache: Cache<LintResult> = new Cache();

export type FormatRequest = TransformRequest & {format: boolean};

export default async function lint(req: FormatRequest): Promise<LintResult> {
  const {ast, sourceText, project, format} = req;

  const query = Cache.buildQuery(req);
  const cached = lintCache.get(query);
  if (cached) {
    return cached;
  }

  if (ast.corrupt) {
    const result: LintResult = {
      suppressions: [],
      diagnostics: [...ast.diagnostics],
      src: req.sourceText,
    };
    lintCache.set(query, result);
    return result;
  }

  let formattedCode = sourceText;
  if (format) {
    // Perform autofixes
    const context = new Context({
      ast,
      project,
      origin: {
        category: 'lint',
      },
    });
    const newAst = program.assert(context.reduce(ast, lintTransforms, {
      frozen: false,
    }));

    const generator = generateJS(newAst, {
      typeAnnotations: true,
      format: 'pretty',
    }, sourceText);
    formattedCode = generator.buf.getCode();
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
    suppressions: context.suppressions,
    diagnostics: [...ast.diagnostics, ...context.diagnostics],
    src: formattedCode,
  };
  lintCache.set(query, result);
  return result;
}
