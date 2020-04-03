/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {
  Tokens,
  space,
  operator,
  word,
  linkedGroups,
  concat,
} from '../../tokens';
import {functionExpression, AnyNode} from '@romejs/js-ast';

export default function FunctionExpression(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = node.type === 'FunctionDeclaration'
    ? node
    : functionExpression.assert(node);

  const tokens: Tokens = [];

  if (node.head.async === true) {
    tokens.push(word('async'));
    tokens.push(space);
  }

  tokens.push(word('function'));

  if (node.head.generator === true) {
    tokens.push(operator('*'));
  }

  if (node.id) {
    tokens.push(space, concat(builder.tokenize(node.id, node)));
  }

  return [
    concat(tokens),
    linkedGroups([concat(builder.tokenize(node.head, node)), space]),
    concat(builder.tokenize(node.body, node)),
  ];
}
