/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {ArrayExpression, arrayExpression, AnyNode} from '@romejs/js-ast';

export default function ArrayExpression(generator: Generator, node: AnyNode) {
  node =
    node.type === 'BindingArrayPattern' ||
    node.type === 'AssignmentArrayPattern'
      ? node
      : arrayExpression.assert(node);

  const elems = node.elements;
  const len = elems.length;

  generator.token('[');
  generator.printInnerComments(node);

  for (let i = 0; i < elems.length; i++) {
    const elem = elems[i];
    if (elem) {
      if (i > 0) {
        generator.space();
      }
      generator.print(elem, node);
      if (i < len - 1) {
        generator.token(',');
      }
    } else {
      // If the array expression ends with a hole, that hole
      // will be ignored by the interpreter, but if it ends with
      // two (or more) holes, we need to write out two (or more)
      // commas so that the resulting code is interpreted with
      // both (all) of the holes.
      generator.token(',');
    }
  }

  if (
    (node.type === 'BindingArrayPattern' ||
      node.type === 'AssignmentArrayPattern') &&
    node.rest !== undefined
  ) {
    if (elems.length > 0) {
      generator.token(',');
      generator.space();
    }

    generator.token('...');
    generator.print(node.rest, node);
  }

  generator.token(']');
}
