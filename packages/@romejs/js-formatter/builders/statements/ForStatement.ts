/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, concat, operator, space, word} from '../../tokens';
import {AnyNode, forStatement} from '@romejs/js-ast';

export default function ForStatement(builder: Builder, node: AnyNode): Tokens {
  node = forStatement.assert(node);

  builder.inForStatementInitCounter++;
  const tokens: Tokens = [
    word('for'),
    space,
    operator('('),
    concat(builder.tokenize(node.init, node)),
    operator(';'),
  ];
  builder.inForStatementInitCounter--;

  if (node.test) {
    tokens.push(space, concat(builder.tokenize(node.test, node)));
  }

  tokens.push(operator(';'));

  if (node.update) {
    tokens.push(space, concat(builder.tokenize(node.update, node)));
  }

  return [
    concat(tokens),
    operator(')'),
    space,
    concat(builder.tokenize(node.body, node)),
  ];
}
