/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, operator, space, word} from '../../tokens';
import {AnyNode, FlowOpaqueType, flowOpaqueType} from '@romejs/js-ast';

export default function FlowOpaqueType(builder: Builder, node: AnyNode): Tokens {
  node = node.type === 'FlowDeclareOpaqueType'
    ? node
    : flowOpaqueType.assert(node);

  let tokens: Tokens = [
    word('opaque'),
    space,
    word('type'),
    space,
    ...builder.tokenize(node.id, node),
    ...builder.tokenize(node.typeParameters, node),
  ];

  if (node.supertype) {
    tokens = [
      ...tokens,
      operator(':'),
      space,
      ...builder.tokenize(node.supertype, node),
    ];
  }

  if (node.impltype) {
    tokens = [
      ...tokens,
      space,
      operator('='),
      space,
      ...builder.tokenize(node.impltype, node),
    ];
  }

  tokens.push(operator(';'));
  return tokens;
}
