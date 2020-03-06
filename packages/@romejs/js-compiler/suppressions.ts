/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Program, AnyComment} from '@romejs/js-ast';
import {DiagnosticSuppressions} from '@romejs/diagnostics';

const SUPPRESSION_START = 'rome-suppress';

function extractSuppressionsFromComment(
  comment: AnyComment,
): undefined | DiagnosticSuppressions {
  const {loc} = comment;
  if (loc === undefined) {
    return undefined;
  }

  const suppressions: DiagnosticSuppressions = [];

  const lines = comment.value.split('\n');
  const cleanLines = lines.map(line => {
    // Trim line and remove leading star
    return line.trim().replace(/\*[\s]/, '');
  });

  for (const line of cleanLines) {
    if (!line.startsWith(SUPPRESSION_START)) {
      continue;
    }

    const categories = line
      .slice(SUPPRESSION_START.length)
      .trim()
      .split(' ');
    const cleanCategories = categories.map(category => category.trim());

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

      suppressions.push({
        category,
        loc,
      });

      if (shouldBreak) {
        break;
      }
    }
  }

  if (suppressions.length === 0) {
    return undefined;
  } else {
    return suppressions;
  }
}

export function extractSuppressionsFromComments(
  comments: Array<AnyComment>,
): DiagnosticSuppressions {
  let suppressions: DiagnosticSuppressions = [];

  for (const comment of comments) {
    const commentSuppressions = extractSuppressionsFromComment(comment);
    if (commentSuppressions !== undefined) {
      suppressions = suppressions.concat(commentSuppressions);
    }
  }

  return suppressions;
}

export function extractSuppressionsFromProgram(
  ast: Program,
): DiagnosticSuppressions {
  return extractSuppressionsFromComments(ast.comments);
}
