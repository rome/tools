/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {
  VariableDeclarationStatement,
  variableDeclarationStatement,
  AnyNode,
} from '@romejs/js-ast';
import {Tokens, operator, word} from '@romejs/js-formatter/tokens';

export default function VariableDeclarationStatement(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = variableDeclarationStatement.assert(node);

  if (node.declare === true && !builder.options.typeAnnotations) {
    return [];
  }

  let tokens: Tokens = [];

  if (node.declare) {
    tokens.push(word('declare'));
  }

  return [...tokens, ...builder.print(node.declaration, node), operator(';')];
}
