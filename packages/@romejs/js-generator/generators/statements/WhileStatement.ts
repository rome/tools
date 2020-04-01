/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, word, operator, space} from '../../tokens';
import {WhileStatement, whileStatement, AnyNode} from '@romejs/js-ast';

export default function WhileStatement(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = whileStatement.assert(node);

  return [
    word('while'),
    space,
    operator('('),
    ...generator.print(node.test, node),
    operator(')'),
    space,
    ...generator.print(node.body, node),
  ];
}
