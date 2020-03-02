/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from '@romejs/js-compiler';
import {AnyNode} from '@romejs/js-ast';

export default {
  name: 'unsafeNegation',
  enter(path: Path): AnyNode {
    const {node} = path;
    if (
      node.type === 'BinaryExpression' &&
      (node.operator === 'in' || node.operator === 'instanceof') &&
      node.left.type === 'UnaryExpression' &&
      node.left.operator === '!'
    ) {
      path.context.addNodeDiagnostic(node, {
        category: 'lint/unsafeNegation',
        message:
          'Unsafe usage of negation operator in left side of binary expression',
      });
    }
    return node;
  },
};
