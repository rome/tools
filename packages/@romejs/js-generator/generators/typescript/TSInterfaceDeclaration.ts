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
import {Generator} from '@romejs/js-generator';
import {Tokens, word, space} from '../../tokens';

export default function TSInterfaceDeclaration(
  generator: Generator,
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
    ...generator.print(node.id, node),
    ...generator.print(node.typeParameters, node),
  ];

  if (node.extends) {
    tokens = [
      ...tokens,
      space,
      word('extends'),
      space,
      generator.printCommaList(node.extends, node),
    ];
  }

  return [...tokens, space, ...generator.print(node.body, node)];
}
