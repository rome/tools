/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSMappedType, tsMappedType, AnyNode} from '@romejs/js-ast';
import {Builder} from '@romejs/js-formatter';
import {Tokens, word, operator, space} from '../../tokens';

export default function TSMappedType(builder: Builder, node: AnyNode): Tokens {
  node = tsMappedType.assert(node);

  let tokens: Tokens = [operator('{'), space];

  if (node.readonly) {
    tokens = [
      ...tokens,
      ...tokenIfPlusMinus(builder, node.readonly),
      word('readonly'),
      space,
    ];
  }

  const {typeParameter} = node;
  tokens = [
    ...tokens,
    operator('['),
    word(typeParameter.name),
    space,
    word('in'),
    space,
    ...builder.tokenize(typeParameter.constraint, typeParameter),
    operator(']'),
  ];

  if (node.optional) {
    tokens = [
      ...tokens,
      ...tokenIfPlusMinus(builder, node.optional),
      operator('?'),
    ];
  }

  return [
    ...tokens,
    operator(':'),
    space,
    ...builder.tokenize(node.typeAnnotation, node),
    space,
    operator('}'),
  ];
}

function tokenIfPlusMinus(builder: Builder, token: string | true): Tokens {
  if (token !== true) {
    return [operator(token)];
  } else {
    return [];
  }
}
