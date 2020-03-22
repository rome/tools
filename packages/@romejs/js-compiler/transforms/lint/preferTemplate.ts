/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from '@romejs/js-compiler';
import {AnyNode} from '@romejs/js-ast';
import {descriptions} from '@romejs/diagnostics';

export default {
  name: 'preferTemplate',
  enter(path: Path): AnyNode {
    const {node} = path;

    if (node.type === 'BinaryExpression' && node.operator === '+' &&
      (node.left.type === 'StringLiteral' && !node.left.value.includes('`') ||
      node.right.type === 'StringLiteral' && !node.right.value.includes('`'))) {
      path.context.addNodeDiagnostic(node, {
        description: descriptions.LINT.PREFER_TEMPLATE,
      });
    }

    return node;
  },
};
