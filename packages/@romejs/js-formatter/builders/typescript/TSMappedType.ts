/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSMappedType} from '@romejs/js-ast';
import {Builder} from '@romejs/js-formatter';
import {Token, concat, space} from '../../tokens';

export default function TSMappedType(
  builder: Builder,
  node: TSMappedType,
): Token {
  const tokens: Array<Token> = ['{', space];

  if (node.readonly) {
    tokens.push(tokenIfPlusMinus(builder, node.readonly), 'readonly', space);
  }

  const {typeParameter} = node;
  tokens.push(
    '[',
    typeParameter.name,
    space,
    'in',
    space,
    builder.tokenize(typeParameter.constraint, typeParameter),
    ']',
  );

  if (node.optional) {
    tokens.push(tokenIfPlusMinus(builder, node.optional), '?');
  }

  if (node.typeAnnotation) {
    tokens.push(':', space, builder.tokenize(node.typeAnnotation, node));
  }

  tokens.push(space, '}');

  return concat(tokens);
}

function tokenIfPlusMinus(builder: Builder, token: string | true): Token {
  if (token !== true) {
    return token;
  } else {
    return '';
  }
}
