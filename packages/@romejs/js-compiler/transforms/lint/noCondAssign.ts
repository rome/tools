/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from '@romejs/js-compiler';
import {AnyNode} from '@romejs/js-ast';

export default {
  name: 'noCondAssign',
  enter(path: Path): AnyNode {
    const {node} = path;

    if (
      (node.type === 'IfStatement' ||
        node.type === 'ForStatement' ||
        node.type === 'WhileStatement' ||
        node.type === 'DoWhileStatement') &&
      node.test &&
      node.test.type === 'AssignmentExpression'
    ) {
      path.context.addNodeDiagnostic(node, {
        category: 'lint/noCondAssign',
        message: 'Cannot assign variable in loop condition',
      });
    }
    return node;
  },
};
