/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {DiagnosticSuppressions, Diagnostics} from '@romejs/diagnostics';
import {TransformRequest} from '../types';
import {lintTransforms} from './rules/index';
import {Cache, CompilerContext} from '@romejs/js-compiler';
import {formatJS} from '@romejs/js-formatter';
import {addSuppressions} from './suppressions';

export type LintResult = {
  diagnostics: Diagnostics;
  suppressions: DiagnosticSuppressions;
  src: string;
};

const lintCache: Cache<LintResult> = new Cache();

export default async function lint(req: TransformRequest): Promise<LintResult> {
  const {ast, sourceText, project, options} = req;

  const query = Cache.buildQuery(req);
  const cached = lintCache.get(query);
  if (cached) {
    return cached;
  }

  // Perform autofixes
  const formatContext = new CompilerContext({
    ref: req.ref,
    sourceText: req.sourceText,
    options,
    ast,
    project,
    frozen: false,
    origin: {
      category: 'lint',
    },
  });

  let formatAst = formatContext.reduceRoot(ast, lintTransforms);
  formatAst = addSuppressions(formatContext, formatAst);
  const generator = formatJS(formatAst, {
    typeAnnotations: true,
    sourceMaps: true,
    format: 'pretty',
    sourceText,
  });
  const formattedCode = generator.getCode();

  // Run lints (could be with the autofixed AST)
  const context = new CompilerContext({
    ref: req.ref,
    sourceText: req.sourceText,
    ast,
    project,
    options,
    origin: {
      category: 'lint',
    },
    frozen: true,
  });
  context.reduceRoot(ast, lintTransforms);

  const diagnostics = context.diagnostics.getDiagnostics();
  const result: LintResult = {
    suppressions: context.suppressions,
    diagnostics: [...ast.diagnostics, ...diagnostics],
    src: formattedCode,
  };
  lintCache.set(query, result);
  return result;
}
