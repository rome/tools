/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {
  VariableDeclarationStatement,
  variableDeclarationStatement,
  AnyNode,
} from '@romejs/js-ast';
import {Tokens, operator, word} from '@romejs/js-generator/tokens';

export default function VariableDeclarationStatement(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = variableDeclarationStatement.assert(node);

  if (node.declare === true && !generator.options.typeAnnotations) {
    return [];
  }

  let tokens: Tokens = [];

  if (node.declare) {
    tokens.push(word('declare'));
  }

  return [...tokens, ...generator.print(node.declaration, node), operator(';')];
}
