/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  getBindingIdentifiers,
  isVariableIdentifier,
} from '@romejs/js-ast-utils';
import {Path, Context} from '@romejs/js-compiler';
import {AnyNode} from '@romejs/js-ast';

function addDiagnostic(name: string, node: AnyNode, context: Context) {
  const identifiers = getBindingIdentifiers(node).map(
    (identifier) => identifier.name,
  );

  if (!identifiers.includes(name)) {
    return;
  }

  context.addNodeDiagnostic(node, {
    category: 'lint/noExAssign',
    message: 'Do not assign to the exception parameter.',
  });
}

export default {
  name: 'noExAssign',
  enter(path: Path): AnyNode {
    const {node, context} = path;

    if (node.type === 'CatchClause' && node.body.type === 'BlockStatement') {
      node.body.body.forEach((item) => {
        if (item.type === 'ExpressionStatement' && item.expression.type ===
            'AssignmentExpression') {
          if (isVariableIdentifier(item.expression.left)) {
            const {name} = item.expression.left;

            addDiagnostic(name, node, context);
          }

          if (item.expression.left.type === 'AssignmentArrayPattern') {
            const {elements} = item.expression.left;

            elements.forEach((element) => {
              if (!element || element && !isVariableIdentifier(element)) {
                return;
              }

              const {name} = element;

              addDiagnostic(name, node, context);
            });
          }
        }
      });
    }

    return node;
  },
};
