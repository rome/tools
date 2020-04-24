/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {AnyNode, variableDeclarationStatement} from '@romejs/js-ast';
import {Tokens, concat, operator, word} from '@romejs/js-formatter/tokens';

export default function VariableDeclarationStatement(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = variableDeclarationStatement.assert(node);

  if (node.declare === true && !builder.options.typeAnnotations) {
    return [];
  }

  const tokens: Tokens = [];

  if (node.declare) {
    tokens.push(word('declare'));
  }

  return [
    concat(tokens),
    concat(builder.tokenize(node.declaration, node)),
    operator(';'),
  ];
}
