/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSMappedType, tsMappedType, AnyNode} from '@romejs/js-ast';
import {Generator} from '@romejs/js-generator';
import {Tokens, word, operator, space} from '../../tokens';

export default function TSMappedType(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = tsMappedType.assert(node);

  let tokens: Tokens = [operator('{'), space];

  if (node.readonly) {
    tokens = [
      ...tokens,
      ...tokenIfPlusMinus(generator, node.readonly),
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
    ...generator.print(typeParameter.constraint, typeParameter),
    operator(']'),
  ];

  if (node.optional) {
    tokens = [
      ...tokens,
      ...tokenIfPlusMinus(generator, node.optional),
      operator('?'),
    ];
  }

  return [
    ...tokens,
    operator(':'),
    space,
    ...generator.print(node.typeAnnotation, node),
    space,
    operator('}'),
  ];
}

function tokenIfPlusMinus(generator: Generator, token: string | true): Tokens {
  if (token !== true) {
    return [operator(token)];
  } else {
    return [];
  }
}
