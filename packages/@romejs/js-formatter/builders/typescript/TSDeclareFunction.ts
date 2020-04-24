/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode, TSDeclareFunction, tsDeclareFunction} from '@romejs/js-ast';
import {Builder} from '@romejs/js-formatter';
import {Tokens, operator, space, word} from '../../tokens';

export default function TSDeclareFunction(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = tsDeclareFunction.assert(node);

  let tokens: Tokens = [];

  if (node.declare) {
    tokens = [word('declare'), space];
  }

  return [
    ...tokens,
    word('function'),
    ...builder.tokenize(node.id, node),
    ...builder.tokenize(node.head, node),
    operator(';'),
  ];
}
