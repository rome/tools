/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Program, AnyComment} from '@romejs/js-ast';
import {
  DiagnosticSuppressions,
  Diagnostics,
  descriptions,
} from '@romejs/diagnostics';

const SUPPRESSION_START = 'rome-suppress';
const PREFIX_MISTAKES = ['@rome-suppress', 'rome-ignore', '@rome-ignore'];

type ExtractedSuppressions = {
  suppressions: DiagnosticSuppressions;
  diagnostics: Diagnostics;
};

function extractSuppressionsFromComment(
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
    if (!line.startsWith(SUPPRESSION_START)) {
      for (const prefix of PREFIX_MISTAKES) {
        if (line.startsWith(prefix)) {
          diagnostics.push({
            description: descriptions.SUPPRESSIONS.PREFIX_TYPO(
              prefix,
              SUPPRESSION_START,
            ),
            location: loc,
          });
        }
      }
      continue;
    }

    const categories = line.slice(SUPPRESSION_START.length).trim().split(' ');
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
