/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, concat, operator, space} from '../../tokens';
import {AnyNode, arrayExpression} from '@romejs/js-ast';

export default function ArrayExpression(
  builder: Builder,
  _node: AnyNode,
): Tokens {
  const node = _node.type === 'BindingArrayPattern' || _node.type ===
    'AssignmentArrayPattern' ? _node : arrayExpression.assert(_node);

  const elems = node.elements;

  const tokens: Tokens = [
    operator('['),
    concat(builder.tokenizeInnerComments(node)),
    builder.tokenizeCommaList(elems, node, {
      trailing: true,
      breakOnNewline: true,
    }),
  ];

  if ((node.type === 'BindingArrayPattern' || node.type ===
      'AssignmentArrayPattern') && node.rest !== undefined) {
    if (elems.length > 0) {
      tokens.push(operator(','));
      tokens.push(space);
    }

    tokens.push(operator('...'), concat(builder.tokenize(node.rest, node)));
  }

  tokens.push(operator(']'));

  return tokens;
}
