/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode, AnyDeclaration} from '@romejs/js-ast';
import parens from './parentheses';
import {ob1Get1, ob1Get0} from '@romejs/ob1';
import {isDeclaration} from '@romejs/js-ast-utils';
import {SourceLocation} from '@romejs/parser-core';

function isOrHasCallExpression(node: AnyNode): boolean {
  if (node.type === 'CallExpression') {
    return true;
  }

  if (node.type === 'ComputedMemberProperty') {
    return isOrHasCallExpression(node.value);
  }

  if (node.type === 'MemberExpression') {
    return isOrHasCallExpression(node.object) || isOrHasCallExpression(
      node.property,
    );
  }

  return false;
}

const EXTRA_LINE_DECLARATION_DENYLIST: Set<AnyDeclaration['type']> = new Set([
  'VariableDeclarationStatement',
  'ImportDeclaration',
  'ExportExternalDeclaration',
  'TSDeclareFunction',
]);

const EXTRA_LINE_ALLOWLIST: Set<AnyNode['type']> = new Set([
  'ObjectMethod',
  'ClassMethod',
]);

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

export function hasExtraLineBetween(node: AnyNode): boolean {
  if (node.type === 'ExportLocalDeclaration' && node.declaration !== undefined) {
    return hasExtraLineBetween(node.declaration);
  }

  if (isDeclaration(node) && !EXTRA_LINE_DECLARATION_DENYLIST.has(node.type)) {
    return true;
  }

  if (EXTRA_LINE_ALLOWLIST.has(node.type)) {
    return true;
  }

  return false;
}

export function getLinesBetween(
  aNode: undefined | AnyNode,
  bNode: undefined | AnyNode,
): Array<number> {
  if (aNode !== undefined && bNode !== undefined && aNode.loc !== undefined &&
        bNode.loc !==
        undefined) {
    const [a, b] = orderLoc(aNode.loc, bNode.loc);
    const lines: Array<number> = [];
    for (let line = ob1Get1(a.end.line); line < ob1Get1(b.start.line); line++) {
      lines.push(line);
    }
    return lines;
  } else {
    return [];
  }
}

export function isMultiLine(node: undefined | AnyNode): boolean {
  if (node !== undefined && node.loc !== undefined) {
    return node.loc.end.line > node.loc.start.line;
  } else {
    return true;
  }
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
