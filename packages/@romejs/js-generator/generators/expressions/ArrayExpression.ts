/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, space, operator} from '../../tokens';
import {ArrayExpression, arrayExpression, AnyNode} from '@romejs/js-ast';

export default function ArrayExpression(
  generator: Generator,
  _node: AnyNode,
): Tokens {
  const node =
    _node.type === 'BindingArrayPattern' ||
    _node.type === 'AssignmentArrayPattern'
      ? _node
      : arrayExpression.assert(_node);

  const elems = node.elements;

  let tokens: Tokens = [
    operator('['),
    ...generator.printInnerComments(node),
    generator.printCommaList(elems, node, {
      trailing: true,
      breakOnNewline: true,
    }),
  ];

  if (
    (node.type === 'BindingArrayPattern' ||
      node.type === 'AssignmentArrayPattern') &&
    node.rest !== undefined
  ) {
    if (elems.length > 0) {
      tokens.push(operator(','));
      tokens.push(space);
    }

    tokens = [...tokens, operator('...'), ...generator.print(node.rest, node)];
  }

  tokens.push(operator(']'));

  return tokens;
}
