/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, space, word, operator} from '../../tokens';
import {FlowTypeParameter, flowTypeParameter, AnyNode} from '@romejs/js-ast';

export default function FlowTypeParameter(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = flowTypeParameter.assert(node);

  let tokens: Tokens = [
    ...generator.print(node.variance, node),
    word(node.name),
  ];

  if (node.bound) {
    tokens = [...tokens, ...generator.print(node.bound, node)];
  }

  if (node.default) {
    return [
      ...tokens,
      space,
      operator('='),
      space,
      ...generator.print(node.default, node),
    ];
  } else {
    return tokens;
  }
}
