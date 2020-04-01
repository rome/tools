/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSEnumMember, tsEnumMember, AnyNode} from '@romejs/js-ast';
import {Generator} from '@romejs/js-generator';
import {Tokens, space, operator} from '../../tokens';

export default function TSEnumMember(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = tsEnumMember.assert(node);

  let tokens: Tokens = generator.print(node.id, node);

  if (node.initializer) {
    tokens = [
      ...tokens,
      space,
      operator('='),
      space,
      ...generator.print(node.initializer, node),
    ];
  }

  return [...tokens, operator(',')];
}
