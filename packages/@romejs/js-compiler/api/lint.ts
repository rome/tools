/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Program} from '@romejs/js-ast';
import {PartialDiagnostics, DiagnosticFilters} from '@romejs/diagnostics';
import {TransformRequest} from '../types';
import {lintTransforms} from '../transforms/lint/index';
import {program} from '@romejs/js-ast';
import {Cache, Context} from '@romejs/js-compiler';
import {generateJS} from '@romejs/js-generator';
import {extractSuppressionsFromProgram} from '../suppressions';

export type LintResult = {
  diagnostics: PartialDiagnostics;
  filters: DiagnosticFilters;
  src: string;
  ast: Program;
};

const lintCache: Cache<LintResult> = new Cache();

export default async function lint(req: TransformRequest): Promise<LintResult> {
  const {ast, sourceText: src, project} = req;

  if (!project.config.lint.enabled) {
    return {
      diagnostics: [],
      filters: extractSuppressionsFromProgram(ast),
      src,
      ast,
    };
  }

  const query = Cache.buildQuery(req);
  const cached = lintCache.get(query);
  if (cached) {
    return cached;
  }

  const context = new Context({
    ast,
    project,
    origin: {
      category: 'lint',
    },
  });
  const newAst = program.assert(context.reduce(ast, lintTransforms));

  let formattedCode = src;
  if (project.config.format.enabled) {
    const generator = generateJS(
      newAst,
      {
        typeAnnotations: true,
      },
      src,
    );
    formattedCode = generator.getCode() + '\n';
  }

  const result: LintResult = {
    filters: extractSuppressionsFromProgram(ast),
    ast: newAst,
    diagnostics: [...ast.diagnostics, ...context.diagnostics],
    src: formattedCode,
  };
  lintCache.set(query, result);
  return result;
}
