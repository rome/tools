/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  Diagnostics,
  DiagnosticSuppressions,
  DiagnosticAdvice,
} from '@romejs/diagnostics';
import {TransformRequest} from '../types';
import {lintTransforms} from '../transforms/lint/index';
import {program} from '@romejs/js-ast';
import {Cache, Context} from '@romejs/js-compiler';
import {formatJS} from '@romejs/js-formatter';
import {Mappings, Mapping} from '@romejs/codec-source-map';
import {get0, Number0, Number1} from '@romejs/ob1';
import stringDiff from '@romejs/string-diff';

function findMapping(
  mappings: Mappings,
  line: Number1,
  column: Number0,
): undefined | Mapping['generated'] {
  for (const {original, generated} of mappings) {
    if (original !== undefined && original.line === line && original.column ===
        column) {
      return generated;
    }
  }

  return undefined;
}

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

  let formattedMappings: undefined | Mappings;
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

    const generator = formatJS(newAst, {
      typeAnnotations: true,
      sourceMaps: true,
      format: 'pretty',
      sourceText,
    });
    formattedCode = generator.getCode();
    formattedMappings = generator.getMappings();
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

  const diagnostics = context.diagnostics.getDiagnostics();

  // If we have a formatted source map then attempt to add a diff to all fixable diagnostics
  if (formattedMappings !== undefined) {
    for (let i = 0; i < diagnostics.length; i++) {
      const diag = diagnostics[i];
      if (!diag.fixable) {
        continue;
      }

      // Get the source text location
      const {start, end} = diag.location;
      if (start === undefined || end === undefined) {
        continue;
      }

      // Try to find the location in the formatted code
      const newStart = findMapping(formattedMappings, start.line, start.column);
      const newEnd = findMapping(formattedMappings, end.line, end.column);
      if (newStart === undefined || newEnd === undefined) {
        continue;
      }

      // Get the source text to compare
      const oldCode = req.sourceText.slice(get0(start.index), get0(end.index));
      const newCode = formattedCode.slice(get0(newStart.index), get0(
        newEnd.index,
      ));

      const advice: DiagnosticAdvice = [...(diag.description.advice || [])];

      advice.push({
        type: 'log',
        category: 'info',
        message: 'Possible fix',
      });

      advice.push({
        type: 'diff',
        diff: stringDiff(oldCode, newCode),
      });

      diagnostics[i] = {
        ...diag,
        description: {
          ...diag.description,
          advice,
        },
      };
    }
  }

  const result: LintResult = {
    suppressions: context.suppressions,
    diagnostics: [...ast.diagnostics, ...diagnostics],
    src: formattedCode,
  };
  lintCache.set(query, result);
  return result;
}
