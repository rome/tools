/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */
import {AnyNode} from '@romejs/js-ast';
import {Path} from '@romejs/js-compiler';

const OPERATORS_TO_CHECK = ['>', '>=', '<', '<=', '==', '===', '!=', '!=='];

function isNegZero(node: AnyNode): boolean {
  return (
    node.type === 'UnaryExpression' &&
    node.operator === '-' &&
    node.argument.type === 'NumericLiteral' &&
    node.argument.value === 0
  );
}

export default {
  name: 'noCompareNegZero',
  enter(path: Path) {
    const {node} = path;

    if (
      node.type === 'BinaryExpression' &&
      OPERATORS_TO_CHECK.includes(node.operator) &&
      (isNegZero(node.left) || isNegZero(node.right))
    ) {
      path.context.addNodeDiagnostic(node, {
        category: 'lint/noCompareNegZero',
        message: `Do not use the '${node.operator}' operator to compare against -0.`,
      });
    }

    return node;
  },
};
