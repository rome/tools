/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode, TSMethodSignature, tsMethodSignature} from '@romejs/js-ast';
import {Builder} from '@romejs/js-formatter';
import {Tokens, concat, operator, space} from '../../tokens';

export default function TSMethodSignature(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = tsMethodSignature.assert(node);

  const tokens: Tokens = [
    ...builder.tokenize(node.key, node),
    ...builder.tokenize(node.meta, node),
  ];

  if (node.returnType) {
    tokens.push(operator(':'));
    tokens.push(space);
    tokens.push(concat(builder.tokenize(node.returnType, node)));
  }

  tokens.push(operator(';'));

  return tokens;
}
