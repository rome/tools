/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from '@romejs/js-compiler';
import {AnyNode} from '@romejs/js-ast';

export default {
  name: 'sparseArray',
  enter(path: Path): AnyNode {
    const {node} = path;

    if (
      node.type === 'BinaryExpression' &&
      node.operator === '+' &&
      (node.left.type === 'StringLiteral' ||
        node.right.type === 'StringLiteral')
    ) {
      path.context.addNodeDiagnostic(node, {
        category: 'lint/preferTemplate',
        message:
          "You're using string concatenation when template literals are preferred",
      });
    }

    return node;
  },
};
