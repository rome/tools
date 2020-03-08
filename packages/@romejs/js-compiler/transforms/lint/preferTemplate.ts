/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from '@romejs/js-compiler';
import {
  AnyNode,
  BinaryExpression,
  templateElement,
  TemplateElement,
  templateLiteral,
} from '@romejs/js-ast';

export default {
  name: 'preferTemplate',
  enter(path: Path): AnyNode {
    const {node} = path;

    if (
      node.type === 'BinaryExpression' &&
      node.operator === '+' &&
      ((node.left.type === 'StringLiteral' && !node.left.value.includes('`')) ||
        (node.right.type === 'StringLiteral' &&
          !node.right.value.includes('`')))
    ) {
      path.context.addNodeDiagnostic(node, {
        category: 'lint/preferTemplate',
        message:
          "You're using string concatenation when template literals are preferred",
      });

      return templateLiteral.create({
        expressions: [node.left, node.right].filter(
          node => node.type !== 'StringLiteral',
        ),
        quasis: nodeToQuasis(node),
      });
    }

    return node;
  },
};

function nodeToQuasis(node: BinaryExpression): TemplateElement[] {
  return [
    templateElement.create({
      cooked: node.left.type === 'StringLiteral' ? node.left.value : '',
      raw: node.left.type === 'StringLiteral' ? node.left.value : '',
      tail: false,
    }),
    templateElement.create({
      cooked: node.right.type === 'StringLiteral' ? node.right.value : '',
      raw: node.right.type === 'StringLiteral' ? node.right.value : '',
      tail: true,
    }),
  ];
}
