/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from '@romejs/js-compiler';
import {AnyNode} from '@romejs/js-ast';

function isBooleanConstructorCall(node: AnyNode) {
  return (
    node &&
    node.type === 'NewExpression' &&
    node.arguments &&
    node.callee.type === 'ReferenceIdentifier' &&
    node.callee.name === 'Boolean'
  );
}

function isConditionalStatement(node: AnyNode) {
  return node.type === 'ConditionalExpression' && node.test;
}

function isInBooleanContext(node: AnyNode) {
  return (
    node.type === 'IfStatement' ||
    node.type === 'DoWhileStatement' ||
    node.type === 'WhileStatement' ||
    (node.type === 'ForStatement' && node.test)
  );
}

function getNode(path: Path) {
  let {node} = path;
  if (isBooleanConstructorCall(node)) {
    let {node} = path.getChildPaths('arguments')[0];
    return node;
  }

  if (isInBooleanContext(node) || isConditionalStatement(node)) {
    let {node} = path.getChildPath('test');
    return node;
  }
}

export default {
  name: 'noExtraBooleanCast',
  enter(path: Path): AnyNode {
    const {context} = path;

    let node = getNode(path);

    if (
      (node &&
        node.type === 'UnaryExpression' &&
        node.operator === '!' &&
        node.argument.type === 'UnaryExpression' &&
        node.argument.operator === '!') ||
      (node &&
        node.type === 'CallExpression' &&
        node.callee.type === 'ReferenceIdentifier' &&
        node.callee.name === 'Boolean')
    ) {
      context.addNodeDiagnostic(node, {
        category: 'lint/noExtraBooleanCast',
        message: `Redundant double negation.`,
      });
    }

    return path.node;
  },
};
