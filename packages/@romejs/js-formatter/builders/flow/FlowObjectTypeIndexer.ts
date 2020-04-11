/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, word, operator, space} from '../../tokens';
import {
  FlowObjectTypeIndexer,
  flowObjectTypeIndexer,
  AnyNode,
} from '@romejs/js-ast';

export default function FlowObjectTypeIndexer(builder: Builder, node: AnyNode) {
  node = flowObjectTypeIndexer.assert(node);

  let tokens: Tokens = [];
  if (node.static === true) {
    tokens.push(word('static'));
    tokens.push(space);
  }

  tokens = [...tokens, ...builder.tokenize(node.variance, node), operator('[')];

  if (node.id !== undefined) {
    tokens = [...tokens, ...builder.tokenize(node.id, node), operator(':')];
  }

  return [
    ...tokens,
    space,
    ...builder.tokenize(node.key, node),
    operator(']'),
    operator(':'),
    space,
    ...builder.tokenize(node.value, node),
  ];
}
