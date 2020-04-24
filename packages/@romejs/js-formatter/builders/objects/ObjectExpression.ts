/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, concat, operator, space} from '../../tokens';
import {AnyNode, objectExpression} from '@romejs/js-ast';

export default function ObjectExpression(
  builder: Builder,
  _node: AnyNode,
): Tokens {
  const node = _node.type === 'BindingObjectPattern' || _node.type ===
    'AssignmentObjectPattern' ? _node : objectExpression.assert(_node);

  const props = node.properties;

  const tokens: Tokens = [
    operator('{'),
    concat(builder.tokenizeInnerComments(node)),
    builder.tokenizeCommaList(props, node, {
      trailing: true,
      breakOnNewline: true,
    }),
  ];

  if ((node.type === 'BindingObjectPattern' || node.type ===
      'AssignmentObjectPattern') && node.rest !== undefined) {
    if (props.length > 0) {
      tokens.push(operator(','), space);
    }

    tokens.push(operator('...'), concat(builder.tokenize(node.rest, node)));
  }

  tokens.push(operator('}'));
  return tokens;
}
