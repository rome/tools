/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, operator, word, space} from '../../tokens';
import {ForStatement, forStatement, AnyNode} from '@romejs/js-ast';

export default function ForStatement(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = forStatement.assert(node);

  generator.inForStatementInitCounter++;
  let tokens: Tokens = [
    word('for'),
    space,
    operator('('),
    ...generator.print(node.init, node),
    operator(';'),
  ];
  generator.inForStatementInitCounter--;

  if (node.test) {
    tokens = [...tokens, space, ...generator.print(node.test, node)];
  }
  tokens.push(operator(';'));

  if (node.update) {
    tokens = [...tokens, space, ...generator.print(node.update, node)];
  }

  return [...tokens, operator(')'), space, ...generator.print(node.body, node)];
}
