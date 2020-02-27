/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode, AnyComment} from '@romejs/js-ast';
import parens from './parentheses';
import {get1} from '@romejs/ob1';

function isOrHasCallExpression(node: AnyNode): boolean {
  if (node.type === 'CallExpression') {
    return true;
  }

  if (node.type === 'ComputedMemberProperty') {
    return isOrHasCallExpression(node.value);
  }

  if (node.type === 'MemberExpression') {
    return (
      isOrHasCallExpression(node.object) || isOrHasCallExpression(node.property)
    );
  }

  return false;
}

export function hasExtraLineBetween(
  a: undefined | AnyNode | AnyComment,
  b: undefined | AnyNode | AnyComment,
): boolean {
  return getLinesBetween(a, b) > 1;
}

export function getLinesBetween(
  a: undefined | AnyNode | AnyComment,
  b: undefined | AnyNode | AnyComment,
): number {
  return a && b && a.loc && b.loc
    ? Math.max(0, get1(b.loc.start.line) - get1(a.loc.end.line))
    : 0;
}

export function isMultiLine(node: undefined | AnyNode): boolean {
  return Boolean(
    node && node.loc ? node.loc.end.line > node.loc.start.line : true,
  );
}

export function needsParens(
  node: AnyNode,
  parent: undefined | AnyNode,
  printStack: Array<AnyNode>,
): undefined | boolean {
  if (!parent) {
    return false;
  }

  if (parent.type === 'NewExpression' && parent.callee === node) {
    if (isOrHasCallExpression(node)) {
      return true;
    }
  }

  const fn = parens.get(node.type);
  return fn ? fn(node, parent, printStack) : undefined;
}
