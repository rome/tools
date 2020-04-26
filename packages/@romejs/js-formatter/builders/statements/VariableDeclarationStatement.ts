/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {VariableDeclarationStatement} from '@romejs/js-ast';
import {Token, concat, space} from '../../tokens';

export default function VariableDeclarationStatement(
  builder: Builder,
  node: VariableDeclarationStatement,
): Token {
  if (node.declare === true && !builder.options.typeAnnotations) {
    return '';
  }

  const tokens: Array<Token> = [];

  if (node.declare) {
    tokens.push('declare', space);
  }

  return concat([concat(tokens), builder.tokenize(node.declaration, node), ';']);
}
