/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from '@romejs/js-compiler';
import {
  AnyNode,
  IfStatement,
  DoWhileStatement,
  WhileStatement,
  ForStatement,
  ConditionalExpression,
} from '@romejs/js-ast';

function isBooleanConstructorCall(node: AnyNode) {
  return (
    node.type === 'NewExpression' &&
    node.callee.type === 'ReferenceIdentifier' &&
    node.callee.name === 'Boolean'
  );
}

function isConditionalStatement(node: AnyNode): node is ConditionalExpression {
  return node.type === 'ConditionalExpression';
}

function isInBooleanContext(
  node: AnyNode,
): node is IfStatement | DoWhileStatement | WhileStatement | ForStatement {
  return (
    node.type === 'IfStatement' ||
    node.type === 'DoWhileStatement' ||
    node.type === 'WhileStatement' ||
    node.type === 'ForStatement'
  );
}

function getNode(path: Path): undefined | AnyNode {
  let {node} = path;

  if (isBooleanConstructorCall(node)) {
    if (node.type === 'NewExpression' && node.arguments.length > 0) {
      return node.arguments[0];
    }
  }

  if (isInBooleanContext(node) || isConditionalStatement(node)) {
    return node.test;
  }
}

export default {
  name: 'noExtraBooleanCast',
  enter(path: Path): AnyNode {
    const {context} = path;

    let node = getNode(path);

    if (node !== undefined) {
      if (
        (node.type === 'UnaryExpression' &&
          node.operator === '!' &&
          node.argument.type === 'UnaryExpression' &&
          node.argument.operator === '!') ||
        (node.type === 'CallExpression' &&
          node.callee.type === 'ReferenceIdentifier' &&
          node.callee.name === 'Boolean')
      ) {
        context.addNodeDiagnostic(node, {
          category: 'lint/noExtraBooleanCast',
          message: `Redundant double negation.`,
        });
      }
    }

    return path.node;
  },
};
