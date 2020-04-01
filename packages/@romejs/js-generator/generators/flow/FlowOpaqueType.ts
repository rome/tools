/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, word, operator, space} from '../../tokens';
import {FlowOpaqueType, flowOpaqueType, AnyNode} from '@romejs/js-ast';

export default function FlowOpaqueType(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node =
    node.type === 'FlowDeclareOpaqueType' ? node : flowOpaqueType.assert(node);

  let tokens: Tokens = [
    word('opaque'),
    space,
    word('type'),
    space,
    ...generator.print(node.id, node),
    ...generator.print(node.typeParameters, node),
  ];

  if (node.supertype) {
    tokens = [
      ...tokens,
      operator(':'),
      space,
      ...generator.print(node.supertype, node),
    ];
  }

  if (node.impltype) {
    tokens = [
      ...tokens,
      space,
      operator('='),
      space,
      ...generator.print(node.impltype, node),
    ];
  }

  tokens.push(operator(';'));
  return tokens;
}
