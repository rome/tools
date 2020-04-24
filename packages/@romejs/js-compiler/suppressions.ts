/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyComment, Program} from '@romejs/js-ast';
import {
  DiagnosticLocation,
  DiagnosticSuppression,
  DiagnosticSuppressionType,
  DiagnosticSuppressions,
  Diagnostics,
  descriptions,
} from '@romejs/diagnostics';
import {Dict} from '@romejs/typescript-helpers';
import {ob1Add} from '@romejs/ob1';

export const SUPPRESSION_NEXT_LINE_START = 'rome-suppress-next-line';
const SUPPRESSION_CURRENT_LINE_START = 'rome-suppress-current-line';

const SUPPRESSION_PREFIX_MISTAKES: Dict<string> = {
  'rome-suppress': SUPPRESSION_NEXT_LINE_START,
  '@rome-suppress': SUPPRESSION_NEXT_LINE_START,
  'rome-ignore': SUPPRESSION_NEXT_LINE_START,
  '@rome-ignore': SUPPRESSION_NEXT_LINE_START,
  '@rome-suppression-next-line': SUPPRESSION_NEXT_LINE_START,
  '@rome-suppression-current-line': SUPPRESSION_CURRENT_LINE_START,
};

type ExtractedSuppressions = {
  suppressions: DiagnosticSuppressions;
  diagnostics: Diagnostics;
};

export function extractSuppressionsFromComment(
  comment: AnyComment,
): undefined | ExtractedSuppressions {
  const {loc} = comment;
  if (loc === undefined) {
    return undefined;
  }

  const suppressedCategories: Set<string> = new Set();
  const diagnostics: Diagnostics = [];
  const suppressions: DiagnosticSuppressions = [];

  const lines = comment.value.split('\n');
  const cleanLines = lines.map((line) => {
    // Trim line and remove leading star
    return line.trim().replace(/\*[\s]/, '');
  });

  for (const line of cleanLines) {
    // Find suppression start
    let suppressionType: undefined | DiagnosticSuppressionType;
    let matchedPrefix: undefined | string;
    if (line.startsWith(SUPPRESSION_CURRENT_LINE_START)) {
      matchedPrefix = SUPPRESSION_CURRENT_LINE_START;
      suppressionType = 'current';
    }
    if (line.startsWith(SUPPRESSION_NEXT_LINE_START)) {
      matchedPrefix = SUPPRESSION_NEXT_LINE_START;
      suppressionType = 'next';
    }

    if (suppressionType === undefined || matchedPrefix === undefined) {
      for (const prefix in SUPPRESSION_PREFIX_MISTAKES) {
        const suggestion = SUPPRESSION_PREFIX_MISTAKES[prefix];
        if (line.startsWith(prefix)) {
          diagnostics.push({
            description: descriptions.SUPPRESSIONS.PREFIX_TYPO(
              prefix,
              suggestion,
            ),
            location: loc,
          });
        }
      }
      continue;
    }

    const lineWithoutPrefix = line.slice(matchedPrefix.length);
    if (lineWithoutPrefix[0] !== ' ') {
      diagnostics.push({
        description: descriptions.SUPPRESSIONS.MISSING_SPACE,
        location: loc,
      });
      continue;
    }

    const categories = lineWithoutPrefix.trim().split(' ');
    const cleanCategories = categories.map((category) => category.trim());

    for (let category of cleanCategories) {
      if (category === '') {
        continue;
      }

      // If a category ends with a colon then all the things that follow it are an explanation
      let shouldBreak = false;
      if (category[category.length - 1] === ':') {
        shouldBreak = true;
        category = category.slice(-1);
      }

      if (suppressedCategories.has(category)) {
        diagnostics.push({
          description: descriptions.SUPPRESSIONS.DUPLICATE(category),
          location: loc,
        });
      } else {
        suppressedCategories.add(category);

        suppressions.push({
          type: suppressionType,
          category,
          loc,
        });
      }

      if (shouldBreak) {
        break;
      }
    }
  }

  if (suppressions.length === 0 && diagnostics.length === 0) {
    return undefined;
  } else {
    return {diagnostics, suppressions};
  }
}

export function extractSuppressionsFromComments(
  comments: Array<AnyComment>,
): ExtractedSuppressions {
  let diagnostics: Diagnostics = [];
  let suppressions: DiagnosticSuppressions = [];

  for (const comment of comments) {
    const result = extractSuppressionsFromComment(comment);
    if (result !== undefined) {
      diagnostics = diagnostics.concat(result.diagnostics);
      suppressions = suppressions.concat(result.suppressions);
    }
  }

  return {suppressions, diagnostics};
}

export function extractSuppressionsFromProgram(
  ast: Program,
): ExtractedSuppressions {
  return extractSuppressionsFromComments(ast.comments);
}

export function matchesSuppression(
  loc: DiagnosticLocation,
  suppression: DiagnosticSuppression,
): boolean {
  const targetLine = suppression.type === 'current'
    ? suppression.loc.end.line
    : ob1Add(suppression.loc.end.line, 1);

  if (loc.filename !== undefined && loc.start !== undefined && loc.filename ===
      suppression.loc.filename && loc.start.line === targetLine) {
    return true;
  }

  return false;
}
