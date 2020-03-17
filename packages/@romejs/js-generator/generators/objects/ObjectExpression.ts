/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {ObjectExpression, objectExpression, AnyNode} from '@romejs/js-ast';

export default function ObjectExpression(generator: Generator, _node: AnyNode) {
  const node =
    _node.type === 'BindingObjectPattern' ||
    _node.type === 'AssignmentObjectPattern'
      ? _node
      : objectExpression.assert(_node);

  generator.multiline(
    node,
    (multiline, node) => {
      const props = node.properties;

      generator.token('{');
      generator.printInnerComments(node);
      generator.printCommaList<typeof props[number]>(props, node, {
        multiline,
        trailing: true,
      });

      if (
        (node.type === 'BindingObjectPattern' ||
          node.type === 'AssignmentObjectPattern') &&
        node.rest !== undefined
      ) {
        if (props.length > 0) {
          if (!multiline) {
            generator.token(',');
          }
          generator.spaceOrNewline(multiline);
        }

        generator.token('...');
        generator.print(node.rest, node);
      }

      if (multiline) {
        generator.buf.removeTrailingNewlines();
        generator.forceNewline();
      }

      generator.token('}');
    },
    {conditions: ['more-than-one-line', 'source-had-multiline']},
  );
}
