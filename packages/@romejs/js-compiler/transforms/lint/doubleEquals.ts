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
  name: 'doubleEquals',
  enter(path: Path): AnyNode {
    const {node, context} = path;

    if (node.type === 'BinaryExpression' && node.right.type !== 'NullLiteral' &&
        node.left.type !== 'NullLiteral') {
      if (node.operator === '!=') {
        context.addNodeDiagnostic(node, descriptions.LINT.NEGATE_DOUBLE_EQUALS);
      }

      if (node.operator === '==') {
        context.addNodeDiagnostic(node, descriptions.LINT.DOUBLE_EQUALS);
      }
    }

    return node;
  },
};
