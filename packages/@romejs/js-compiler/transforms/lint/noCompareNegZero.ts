/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */
import {AnyNode} from '@romejs/js-ast';
import {Path} from '@romejs/js-compiler';
import {template} from '@romejs/js-ast-utils';
import {descriptions} from '@romejs/diagnostics';

const OPERATORS_TO_CHECK = ['>', '>=', '<', '<=', '==', '===', '!=', '!=='];

function isNegZero(node: AnyNode): boolean {
  return node.type === 'UnaryExpression' && node.operator === '-' &&
    node.argument.type === 'NumericLiteral' && node.argument.value === 0;
}

export default {
  name: 'noCompareNegZero',
  enter(path: Path) {
    const {node} = path;

    if (node.type === 'BinaryExpression' && OPERATORS_TO_CHECK.includes(
      node.operator,
    ) && (isNegZero(node.left) || isNegZero(node.right))) {
      const {suppressed} = path.context.addNodeDiagnostic(
        node,
        descriptions.LINT.NO_COMPARE_NEG_ZERO(node.operator),
      );
      if (!suppressed && node.operator === '===') {
        return template.expression`Object.is(${node.left}, ${node.right})`;
      }
    }

    return node;
  },
};
