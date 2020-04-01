/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, space, operator} from '../../tokens';
import {ObjectExpression, objectExpression, AnyNode} from '@romejs/js-ast';

export default function ObjectExpression(
  generator: Generator,
  _node: AnyNode,
): Tokens {
  const node =
    _node.type === 'BindingObjectPattern' ||
    _node.type === 'AssignmentObjectPattern'
      ? _node
      : objectExpression.assert(_node);

  const props = node.properties;

  let tokens: Tokens = [
    operator('{'),
    ...generator.printInnerComments(node),
    generator.printCommaList(props, node, {
      trailing: true,
      breakOnNewline: true,
    }),
  ];

  if (
    (node.type === 'BindingObjectPattern' ||
      node.type === 'AssignmentObjectPattern') &&
    node.rest !== undefined
  ) {
    if (props.length > 0) {
      tokens = [...tokens, operator(','), space];
    }

    tokens = [...tokens, operator('...'), ...generator.print(node.rest, node)];
  }

  tokens.push(operator('}'));
  return tokens;
}
