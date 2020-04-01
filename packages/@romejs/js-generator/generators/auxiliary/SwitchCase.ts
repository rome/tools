/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {SwitchCase, switchCase, AnyNode} from '@romejs/js-ast';
import {
  word,
  space,
  newline,
  operator,
  indent,
  Tokens,
} from '@romejs/js-generator/tokens';

export default function SwitchCase(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = switchCase.assert(node);

  let tokens: Tokens = [];

  if (node.test) {
    tokens = [
      word('case'),
      space,
      ...generator.print(node.test, node),
      operator(':'),
    ];
  } else {
    tokens = [word('default'), operator(':')];
  }

  tokens.push(newline);

  const {consequent} = node;
  if (consequent.length === 1 && consequent[0].type === 'BlockStatement') {
    tokens = tokens.concat(generator.print(consequent[0], node));
  } else if (consequent.length > 0) {
    tokens.push(indent(generator.printStatementList(consequent, node)));
  }

  return tokens;
}
