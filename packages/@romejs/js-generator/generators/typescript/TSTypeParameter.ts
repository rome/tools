/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSTypeParameter, tsTypeParameter, AnyNode} from '@romejs/js-ast';
import {Generator} from '@romejs/js-generator';
import {Tokens, space, word, operator} from '../../tokens';

export default function TSTypeParameter(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = tsTypeParameter.assert(node);

  let tokens: Tokens = [word(node.name)];

  if (node.constraint) {
    tokens = [
      ...tokens,
      space,
      word('extends'),
      space,
      ...generator.print(node.constraint, node),
    ];
  }

  if (node.default) {
    tokens = [
      ...tokens,
      space,
      operator('='),
      space,
      ...generator.print(node.default, node),
    ];
  }

  return tokens;
}
