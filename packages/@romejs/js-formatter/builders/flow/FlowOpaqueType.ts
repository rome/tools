/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Token, concat, space} from '../../tokens';
import {FlowDeclareOpaqueType, FlowOpaqueType} from '@romejs/js-ast';

export default function FlowOpaqueType(
  builder: Builder,
  node: FlowDeclareOpaqueType | FlowOpaqueType,
): Token {
  const tokens: Array<Token> = [
    'opaque',
    space,
    'type',
    space,
    builder.tokenize(node.id, node),
    builder.tokenize(node.typeParameters, node),
  ];

  if (node.supertype) {
    tokens.push(':', space, builder.tokenize(node.supertype, node));
  }

  if (node.impltype) {
    tokens.push(space, '=', space, builder.tokenize(node.impltype, node));
  }

  tokens.push(';');

  return concat(tokens);
}
