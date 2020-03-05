/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from '@romejs/js-compiler';
import {AnyNode, templateElement, TemplateElement, TemplateLiteral, templateLiteral} from '@romejs/js-ast';

const nodeToQuasi = (fromConcat: string, isTail: boolean): TemplateElement => (
  fromConcat === 'StringLiteral' ? 
  templateElement.create({ 
    cooked: fromConcat,
    raw: fromConcat,
    tail: isTail
  }) :
  templateElement.create({
    cooked: '',
    raw: '',
    tail: isTail
  })
)

export default {
  name: 'preferTemplate',
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

      return templateLiteral.create({
      expressions: [node.left, node.right].filter(node => node.type !== 'StringLiteral'),
      quasis:  [{value: node.left, side: 'left'}, {value: node.right, side: 'right'}].map(quasi => 
        nodeToQuasi(quasi.value.type, quasi.side === 'right'))
      })
    }

    return node;
  },
};
