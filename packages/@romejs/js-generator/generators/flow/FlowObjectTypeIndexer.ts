/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, word, operator, space} from '../../tokens';
import {
  FlowObjectTypeIndexer,
  flowObjectTypeIndexer,
  AnyNode,
} from '@romejs/js-ast';

export default function FlowObjectTypeIndexer(
  generator: Generator,
  node: AnyNode,
) {
  node = flowObjectTypeIndexer.assert(node);

  let tokens: Tokens = [];
  if (node.static === true) {
    tokens.push(word('static'));
    tokens.push(space);
  }

  tokens = [...tokens, ...generator.print(node.variance, node), operator('[')];

  if (node.id !== undefined) {
    tokens = [...tokens, ...generator.print(node.id, node), operator(':')];
  }

  return [
    ...tokens,
    space,
    ...generator.print(node.key, node),
    operator(']'),
    operator(':'),
    space,
    ...generator.print(node.value, node),
  ];
}
