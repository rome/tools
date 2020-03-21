/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from '@romejs/js-compiler';
import {AnyNode, unaryExpression} from '@romejs/js-ast';

export default {
  name: 'unsafeNegation',
  enter(path: Path): AnyNode {
    const {node} = path;

    if (node.type === 'BinaryExpression' && (node.operator === 'in' ||
    node.operator === 'instanceof') && node.left.type === 'UnaryExpression' &&
      node.left.operator === '!') {
      path.context.addNodeDiagnostic(node, {
        fixable: true,
        category: 'lint/unsafeNegation',
        message: 'Unsafe usage of negation operator in left side of binary expression',
      });

      return unaryExpression.create({
        operator: node.left.operator,
        argument: {
          ...node,
          left: node.left.argument,
        },
      });
    }

    return node;
  },
};
