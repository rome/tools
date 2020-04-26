/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSDeclareFunction} from '@romejs/js-ast';
import {Builder} from '@romejs/js-formatter';
import {Token, concat, space} from '../../tokens';

export default function TSDeclareFunction(
  builder: Builder,
  node: TSDeclareFunction,
): Token {
  let tokens: Array<Token> = [];

  if (node.declare) {
    tokens.push('declare', space);
  }

  return concat([
    concat(tokens),
    'function',
    space,
    builder.tokenize(node.id, node),
    builder.tokenize(node.head, node),
    ';',
  ]);
}
