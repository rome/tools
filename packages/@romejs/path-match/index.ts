/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {PatternNode} from './types';
import {parsePattern} from './parse';
import match from './match';
import {PathSegments, AbsoluteFilePath} from '@romejs/path';

export type PathPatterns = Array<PatternNode>;
export type PathPattern = PatternNode;

export {parsePattern as parsePathPattern};

export function matchPath(
  path: AbsoluteFilePath,
  patternNode: PatternNode,
  cwdSegs?: PathSegments,
): boolean {
  const matches = match(path.getSegments(), patternNode, cwdSegs);

  if (patternNode.negate) {
    return !matches;
  } else {
    return matches;
  }
}

function getGreater(pattern: PathPattern, num: number): number {
  if (pattern.segments.length > num) {
    return pattern.segments.length;
  } else {
    return num;
  }
}

export function matchPathPatterns(
  path: AbsoluteFilePath,
  patterns: PathPatterns,
  cwd?: AbsoluteFilePath,
): boolean {
  // Bail out if there are no patterns
  if (patterns.length === 0) {
    return false;
  }

  let matches = 0;
  let notMatches = 0;

  const pathSegments = path.getSegments();
  const cwdSegs = cwd === undefined ? undefined : cwd.getSegments();

  // Run all negate patterns first
  for (const pattern of patterns) {
    // No point in matching an empty pattern, could just contain a comment
    if (pattern.segments.length === 0) {
      continue;
    }

    if (pattern.negate) {
      if (match(pathSegments, {...pattern, negate: false}, cwdSegs)) {
        notMatches = getGreater(pattern, notMatches);
      }
    } else {
      if (match(pathSegments, pattern, cwdSegs)) {
        matches = getGreater(pattern, matches);
      }
    }
  }

  return matches > 0 && matches > notMatches;
}
