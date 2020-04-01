/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, space, operator, word, linkedGroups} from '../../tokens';
import {FunctionExpression, functionExpression, AnyNode} from '@romejs/js-ast';

export default function FunctionExpression(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = node.type === 'FunctionDeclaration'
    ? node
    : functionExpression.assert(node);

  let tokens: Tokens = [];

  if (node.head.async === true) {
    tokens.push(word('async'));
    tokens.push(space);
  }

  tokens.push(word('function'));

  if (node.head.generator === true) {
    tokens.push(operator('*'));
  }

  if (node.id) {
    tokens = [...tokens, space, ...generator.print(node.id, node)];
  }

  return [
    ...tokens,
    linkedGroups([...generator.print(node.head, node), space]),
    ...generator.print(node.body, node),
  ];
}
