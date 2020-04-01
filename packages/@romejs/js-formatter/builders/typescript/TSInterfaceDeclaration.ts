/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  TSInterfaceDeclaration,
  tsInterfaceDeclaration,
  AnyNode,
} from '@romejs/js-ast';
import {Builder} from '@romejs/js-formatter';
import {Tokens, word, space} from '../../tokens';

export default function TSInterfaceDeclaration(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = tsInterfaceDeclaration.assert(node);

  let tokens: Tokens = [];

  if (node.declare) {
    tokens = [word('declare'), space];
  }

  tokens = [
    ...tokens,
    word('interface'),
    space,
    ...builder.print(node.id, node),
    ...builder.print(node.typeParameters, node),
  ];

  if (node.extends) {
    tokens = [
      ...tokens,
      space,
      word('extends'),
      space,
      builder.printCommaList(node.extends, node),
    ];
  }

  return [...tokens, space, ...builder.print(node.body, node)];
}
