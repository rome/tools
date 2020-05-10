/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from '@romejs/js-ast';
import {ob1Get0, ob1Get1} from '@romejs/ob1';
import {SourceLocation} from '@romejs/parser-core';
import parentheses from './parentheses';

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

function orderLoc(
  a: SourceLocation,
  b: SourceLocation,
): [SourceLocation, SourceLocation] {
  if (ob1Get0(a.end.index) < ob1Get0(b.start.index)) {
    return [a, b];
  } else {
    return [b, a];
  }
}

export function getLinesBetween(aNode: AnyNode, bNode: AnyNode): number {
  if (aNode.loc && bNode.loc) {
    const [a, b] = orderLoc(aNode.loc, bNode.loc);
    return ob1Get1(b.start.line) - ob1Get1(a.end.line);
  } else {
    return 0;
  }
}

export function needsParens(
  node: AnyNode,
  parent: undefined | AnyNode,
  printStack: Array<AnyNode>,
): boolean {
  if (!parent) {
    return false;
  }

  if (parent.type === 'NewExpression' && parent.callee === node) {
    if (isOrHasCallExpression(node)) {
      return true;
    }
  }

  const fn = parentheses.get(node.type);
  return fn ? fn(node, parent, printStack) : false;
}

export function isOnSameLine(aNode: AnyNode, bNode: AnyNode): boolean {
  if (aNode.loc && bNode.loc) {
    return aNode.loc.end.line === bNode.loc.start.line;
  } else {
    return false;
  }
}
