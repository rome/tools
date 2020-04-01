/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, word, space} from '../../tokens';
import {
  FlowInterfaceDeclaration,
  flowInterfaceDeclaration,
  AnyNode,
} from '@romejs/js-ast';

export default function FlowInterfaceDeclaration(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = node.type === 'FlowDeclareInterface'
    ? node
    : flowInterfaceDeclaration.assert(node);

  return [word('interface'), space, ..._interfaceish(generator, node)];
}

export function _interfaceish(generator: Generator, node: AnyNode): Tokens {
  node = node.type === 'FlowDeclareInterface' || node.type ===
    'FlowDeclareClass' ? node : flowInterfaceDeclaration.assert(node);

  let tokens: Tokens = [
    ...generator.print(node.id, node),
    ...generator.print(node.typeParameters, node),
  ];

  if (node.extends.length > 0) {
    tokens = [
      ...tokens,
      space,
      word('extends'),
      space,
      generator.printCommaList(node.extends, node),
    ];
  }

  if (node.mixins.length > 0) {
    tokens = [
      ...tokens,
      space,
      word('mixins'),
      space,
      generator.printCommaList(node.mixins, node),
    ];
  }

  return [...tokens, space, ...generator.print(node.body, node)];
}
