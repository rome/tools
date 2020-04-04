/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Token, concat, space} from '../../tokens';
import {FlowTypeParameter} from '@romejs/js-ast';

export default function FlowTypeParameter(
  builder: Builder,
  node: FlowTypeParameter,
): Token {
  const tokens: Array<Token> = [
    builder.tokenize(node.variance, node),
    node.name,
  ];

  if (node.bound) {
    tokens.push(builder.tokenize(node.bound, node));
  }

  if (node.default) {
    return concat([
      concat(tokens),
      space,
      '=',
      space,
      builder.tokenize(node.default, node),
    ]);
  } else {
    return concat(tokens);
  }
}
