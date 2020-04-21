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
  name: 'noExAssign',
  enter(path: Path): AnyNode {
    const {node} = path;

    if (node.type === 'CatchClause') {
      for (const bodyNode of node.body.body) {
        if (bodyNode.type === 'ExpressionStatement' &&
                  bodyNode.expression.type ===
                  'AssignmentExpression' && node.param.type ===
                'BindingIdentifier' &&
              bodyNode.expression.left.type === 'AssignmentIdentifier' &&
              node.param.name ===
              bodyNode.expression.left.name) {
          path.context.addNodeDiagnostic(node, descriptions.LINT.NO_EX_ASSIGN);
        }
      }
    }

    return node;
  },
};
