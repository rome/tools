/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, space, word, operator} from '../../tokens';
import {FlowTypeParameter, flowTypeParameter, AnyNode} from '@romejs/js-ast';

export default function FlowTypeParameter(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = flowTypeParameter.assert(node);

  let tokens: Tokens = [
    ...builder.tokenize(node.variance, node),
    word(node.name),
  ];

  if (node.bound) {
    tokens = [...tokens, ...builder.tokenize(node.bound, node)];
  }

  if (node.default) {
    return [
      ...tokens,
      space,
      operator('='),
      space,
      ...builder.tokenize(node.default, node),
    ];
  } else {
    return tokens;
  }
}
