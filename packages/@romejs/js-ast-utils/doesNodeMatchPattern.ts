/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from '@romejs/js-ast';
import getNodeReferenceParts from './getNodeReferenceParts';

// TODO make this accept multiple matches
export default function doesNodeMatchPattern(
  member: AnyNode,
  match: string | Array<string>,
  allowPartial: boolean = false,
): boolean {
  // Not a member expression
  if (member.type !== 'MemberExpression' && member.type !== 'Identifier') {
    return false;
  }

  const expectedParts: Array<string> = Array.isArray(match)
    ? match.slice()
    : match.split('.');

  const [bailed, actualParts] = getNodeReferenceParts(member);

  // Bailed will be true if we were unable to derive a name for one of the parts
  if (bailed && !allowPartial) {
    return false;
  }

  // If there's less parts than the amount we expect then it's never going to match
  if (actualParts.length < expectedParts.length) {
    return false;
  }

  // I there's more parts than we expect and we weren't passed the allowPartial flag then it's never going to match either
  if (allowPartial === false && actualParts.length > expectedParts.length) {
    return false;
  }

  // Loop over the parts we received and match them
  while (actualParts.length > 0) {
    // If we have no more expected parts then return based on if we allow partial matches
    if (expectedParts.length === 0) {
      return allowPartial;
    }

    const actual = actualParts.shift();
    const expected = expectedParts.shift();

    // A star part can accept anything
    if (expected === '*') {
      continue;
    }

    // A double star will eat as many parts from 'actual until we find the next expected part
    if (expected === '**') {
      const next = expectedParts.shift();

      if (next === '*' || next === '**') {
        throw new Error(
          `The next expected part was ${next} but this isn't allowed since we're processing a double star`,
        );
      }

      let found = false;

      while (actualParts.length > 0) {
        const actual = actualParts.shift();
        if (actual === next) {
          found = true;
          break;
        }
      }

      if (found) {
        continue;
      } else {
        return false;
      }
    }

    if (expected !== actual) {
      return false;
    }
  }

  return true;
}
