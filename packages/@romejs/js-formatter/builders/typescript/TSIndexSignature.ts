/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSIndexSignature, tsIndexSignature, AnyNode} from '@romejs/js-ast';
import {Builder} from '@romejs/js-formatter';
import {Tokens, operator, word, space} from '../../tokens';

export default function TSIndexSignature(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = tsIndexSignature.assert(node);

  let tokens: Tokens = [];
  if (node.readonly) {
    tokens.push(word('readonly'));
    tokens.push(space);
  }

  return [
    ...tokens,
    operator('['),
    ...builder.print(node.key, node),
    operator(']'),
    operator(':'),
    space,
    ...builder.print(node.typeAnnotation, node),
    operator(';'),
  ];
}
