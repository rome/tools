/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path, TransformExitResult} from '@romejs/js-compiler';
import {
  assignmentExpression,
  memberExpression,
  referenceIdentifier,
} from '@romejs/js-ast';
import {descriptions} from '@romejs/diagnostics';

export default {
  name: 'noDelete',
  enter(path: Path): TransformExitResult {
    const {context, node} = path;

    if (
      node.type === 'UnaryExpression' &&
      node.operator === 'delete' &&
      node.argument.type === 'MemberExpression'
    ) {
      const left = node.argument;
      return context.addFixableDiagnostic(
        {
          old: node,
          fixed: assignmentExpression.create(
            {
              operator: '=',
              left: memberExpression.create({
                object: left.object,
                property: left.property,
              }),
              right: referenceIdentifier.create({
                name: 'undefined',
              }),
            },
            node,
          ),
        },
        descriptions.LINT.NO_DELETE,
      );
    }

    return node;
  },
};
