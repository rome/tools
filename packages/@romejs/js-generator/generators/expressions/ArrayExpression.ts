/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {ArrayExpression, arrayExpression, AnyNode} from '@romejs/js-ast';

export default function ArrayExpression(generator: Generator, _node: AnyNode) {
  const node =
    _node.type === 'BindingArrayPattern' ||
    _node.type === 'AssignmentArrayPattern'
      ? _node
      : arrayExpression.assert(_node);

  generator.multiline(
    node,
    (multiline, node) => {
      const elems = node.elements;

      generator.token('[');
      generator.printInnerComments(node);

      generator.printCommaList<NonNullable<typeof elems[number]>>(elems, node, {
        multiline,
        trailing: true,
      });

      if (
        (node.type === 'BindingArrayPattern' ||
          node.type === 'AssignmentArrayPattern') &&
        node.rest !== undefined
      ) {
        if (elems.length > 0) {
          generator.token(',');
          generator.spaceOrNewline(multiline);
        }

        generator.token('...');
        generator.print(node.rest, node);
      }

      if (multiline) {
        generator.buf.removeTrailingNewlines();
        generator.forceNewline();
      }

      generator.token(']');
    },
    {conditions: ['more-than-one-line', 'source-had-multiline']},
  );
}
