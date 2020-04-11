/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from '@romejs/js-compiler';
import {AnyNode, templateLiteral, templateElement} from '@romejs/js-ast';
import {descriptions} from '@romejs/diagnostics';

export default {
  name: 'preferTemplate',
  enter(path: Path): AnyNode {
    const {node} = path;

    if (node.type === 'BinaryExpression' && node.operator === '+' &&
        (node.left.type ===
              'StringLiteral' &&
            !node.left.value.includes('`') ||
          node.right.type === 'StringLiteral' && !node.right.value.includes('`'))) {
      const {suppressed} = path.context.addNodeDiagnostic(
        node,
        descriptions.LINT.PREFER_TEMPLATE,
      );

      if (!suppressed) {
        if (node.right.type === 'StringLiteral') {
          const quasis = [
            templateElement.create({
              raw: '',
              cooked: '',
            }),
            templateElement.create({
              raw: node.right.value,
              cooked: node.right.value,
            }),
          ];
          const expressions = [node.left];
          return templateLiteral.create({
            expressions,
            quasis,
            loc: node.loc,
          });
        }
        if (node.left.type === 'StringLiteral') {
          const quasis = [
            templateElement.create({
              raw: node.left.value,
              cooked: node.left.value,
            }),
            templateElement.create({
              raw: '',
              cooked: '',
            }),
          ];
          const expressions = [node.right];
          return templateLiteral.create({
            expressions,
            quasis,
            loc: node.loc,
          });
        }
      }
    }

    return node;
  },
};
