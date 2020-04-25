/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Token, concat, space} from '../../tokens';
import {YieldExpression} from '@romejs/js-ast';

export default function YieldExpression(
  builder: Builder,
  node: YieldExpression,
): Token {
  const tokens: Array<Token> = ['yield'];

  if (node.delegate === true) {
    tokens.push('*');
  }

  if (node.argument) {
    tokens.push(space, builder.tokenize(node.argument, node));
  }

  return concat(tokens);
}
