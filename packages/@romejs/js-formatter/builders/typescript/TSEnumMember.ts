/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSEnumMember, tsEnumMember, AnyNode} from '@romejs/js-ast';
import {Builder} from '@romejs/js-formatter';
import {Tokens, space, operator} from '../../tokens';

export default function TSEnumMember(builder: Builder, node: AnyNode): Tokens {
  node = tsEnumMember.assert(node);

  let tokens: Tokens = builder.tokenize(node.id, node);

  if (node.initializer) {
    tokens = [
      ...tokens,
      space,
      operator('='),
      space,
      ...builder.tokenize(node.initializer, node),
    ];
  }

  return [...tokens, operator(',')];
}
