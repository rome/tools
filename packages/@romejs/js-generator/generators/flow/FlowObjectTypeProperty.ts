/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, word, space, operator} from '../../tokens';
import {
  FlowObjectTypeProperty,
  flowObjectTypeProperty,
  AnyNode,
} from '@romejs/js-ast';

export default function FlowObjectTypeProperty(
  generator: Generator,
  node: AnyNode,
) {
  node = flowObjectTypeProperty.assert(node);

  let tokens: Tokens = [];

  if (node.static === true) {
    tokens.push(word('static'));
    tokens.push(space);
  }

  tokens = [
    ...tokens,
    ...generator.print(node.variance, node),
    ...generator.print(node.key, node),
  ];

  if (node.optional === true) {
    tokens.push(operator('?'));
  }

  return [
    ...tokens,
    operator(':'),
    space,
    ...generator.print(node.value, node),
  ];
}
