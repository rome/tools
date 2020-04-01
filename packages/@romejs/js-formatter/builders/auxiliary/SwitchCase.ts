/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {SwitchCase, switchCase, AnyNode} from '@romejs/js-ast';
import {
  word,
  space,
  newline,
  operator,
  indent,
  Tokens,
} from '@romejs/js-formatter/tokens';

export default function SwitchCase(builder: Builder, node: AnyNode): Tokens {
  node = switchCase.assert(node);

  let tokens: Tokens = [];

  if (node.test) {
    tokens = [
      word('case'),
      space,
      ...builder.tokenize(node.test, node),
      operator(':'),
    ];
  } else {
    tokens = [word('default'), operator(':')];
  }

  tokens.push(newline);

  const {consequent} = node;
  if (consequent.length === 1 && consequent[0].type === 'BlockStatement') {
    tokens = tokens.concat(builder.tokenize(consequent[0], node));
  } else if (consequent.length > 0) {
    tokens.push(indent(builder.tokenizeStatementList(consequent, node)));
  }

  return tokens;
}
