/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, space, word} from '../../tokens';
import {TryStatement, tryStatement, AnyNode} from '@romejs/js-ast';

export default function TryStatement(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = tryStatement.assert(node);

  const tokens: Tokens = [
    word('try'),
    space,
    ...generator.print(node.block, node),
    space,
    ...generator.print(node.handler, node),
  ];

  if (node.finalizer) {
    return [
      ...tokens,
      space,
      word('finally'),
      space,
      ...generator.print(node.finalizer, node),
    ];
  } else {
    return tokens;
  }
}
