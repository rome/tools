/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, space, word} from '../../tokens';
import {
  AnyNode,
  FlowInterfaceDeclaration,
  flowInterfaceDeclaration,
} from '@romejs/js-ast';

export default function FlowInterfaceDeclaration(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = node.type === 'FlowDeclareInterface'
    ? node
    : flowInterfaceDeclaration.assert(node);

  return [word('interface'), space, ..._interfaceish(builder, node)];
}

export function _interfaceish(builder: Builder, node: AnyNode): Tokens {
  node = node.type === 'FlowDeclareInterface' || node.type ===
    'FlowDeclareClass' ? node : flowInterfaceDeclaration.assert(node);

  let tokens: Tokens = [
    ...builder.tokenize(node.id, node),
    ...builder.tokenize(node.typeParameters, node),
  ];

  if (node.extends.length > 0) {
    tokens = [
      ...tokens,
      space,
      word('extends'),
      space,
      builder.tokenizeCommaList(node.extends, node),
    ];
  }

  if (node.mixins.length > 0) {
    tokens = [
      ...tokens,
      space,
      word('mixins'),
      space,
      builder.tokenizeCommaList(node.mixins, node),
    ];
  }

  return [...tokens, space, ...builder.tokenize(node.body, node)];
}
