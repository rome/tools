/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, space, word} from '../../tokens';
import {TryStatement, tryStatement, AnyNode} from '@romejs/js-ast';

export default function TryStatement(builder: Builder, node: AnyNode): Tokens {
  node = tryStatement.assert(node);

  const tokens: Tokens = [
    word('try'),
    space,
    ...builder.print(node.block, node),
    space,
    ...builder.print(node.handler, node),
  ];

  if (node.finalizer) {
    return [
      ...tokens,
      space,
      word('finally'),
      space,
      ...builder.print(node.finalizer, node),
    ];
  } else {
    return tokens;
  }
}
