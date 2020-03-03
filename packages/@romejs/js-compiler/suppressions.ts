/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Program, AnyComment} from '@romejs/js-ast';
import {DiagnosticFilterJSON} from '@romejs/diagnostics';
import {add} from '@romejs/ob1';

const SUPPRESSION_START = 'rome-suppress';

function extractFiltersFromComment(
  comment: AnyComment,
): undefined | Array<DiagnosticFilterJSON> {
  const {loc} = comment;
  if (loc === undefined) {
    return undefined;
  }

  const targetLine = add(loc.end.line, 1);
  const filters: Array<DiagnosticFilterJSON> = [];

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

    for (const category of cleanCategories) {
      if (category === '') {
        continue;
      }

      filters.push({
        filename: loc.filename,
        category,
        line: targetLine,
      });
    }
  }

  if (filters.length === 0) {
    return undefined;
  } else {
    return filters;
  }
}

export function extractSuppressionsFromComments(
  comments: Array<AnyComment>,
): Array<DiagnosticFilterJSON> {
  let filters: Array<DiagnosticFilterJSON> = [];

  for (const comment of comments) {
    const commentFilters = extractFiltersFromComment(comment);
    if (commentFilters !== undefined) {
      filters = filters.concat(commentFilters);
    }
  }

  return filters;
}

export function extractSuppressionsFromProgram(
  ast: Program,
): Array<DiagnosticFilterJSON> {
  return extractSuppressionsFromComments(ast.comments);
}
