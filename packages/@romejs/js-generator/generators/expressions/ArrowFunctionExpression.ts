/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, operator, word, space} from '../../tokens';
import {
  ArrowFunctionExpression,
  arrowFunctionExpression,
  AnyNode,
} from '@romejs/js-ast';

export default function ArrowFunctionExpression(
  generator: Generator,
  node: AnyNode,
) {
  node = arrowFunctionExpression.assert(node);

  const tokens: Tokens = [];

  if (node.head.async === true) {
    tokens.push(word('async'));
    tokens.push(space);
  }

  return [
    ...tokens,
    ...generator.print(node.head, node),
    space,
    operator('=>'),
    space,
    ...generator.print(node.body, node),
  ];
}
