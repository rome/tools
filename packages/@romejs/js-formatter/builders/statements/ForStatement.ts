/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, operator, word, space} from '../../tokens';
import {ForStatement, forStatement, AnyNode} from '@romejs/js-ast';

export default function ForStatement(builder: Builder, node: AnyNode): Tokens {
  node = forStatement.assert(node);

  builder.inForStatementInitCounter++;
  let tokens: Tokens = [
    word('for'),
    space,
    operator('('),
    ...builder.print(node.init, node),
    operator(';'),
  ];
  builder.inForStatementInitCounter--;

  if (node.test) {
    tokens = [...tokens, space, ...builder.print(node.test, node)];
  }
  tokens.push(operator(';'));

  if (node.update) {
    tokens = [...tokens, space, ...builder.print(node.update, node)];
  }

  return [...tokens, operator(')'), space, ...builder.print(node.body, node)];
}
