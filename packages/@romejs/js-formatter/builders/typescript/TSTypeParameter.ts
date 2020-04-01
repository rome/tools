/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSTypeParameter, tsTypeParameter, AnyNode} from '@romejs/js-ast';
import {Builder} from '@romejs/js-formatter';
import {Tokens, space, word, operator} from '../../tokens';

export default function TSTypeParameter(builder: Builder, node: AnyNode): Tokens {
  node = tsTypeParameter.assert(node);

  let tokens: Tokens = [word(node.name)];

  if (node.constraint) {
    tokens = [
      ...tokens,
      space,
      word('extends'),
      space,
      ...builder.print(node.constraint, node),
    ];
  }

  if (node.default) {
    tokens = [
      ...tokens,
      space,
      operator('='),
      space,
      ...builder.print(node.default, node),
    ];
  }

  return tokens;
}
