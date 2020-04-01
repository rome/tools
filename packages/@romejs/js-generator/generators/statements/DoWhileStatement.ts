/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, word, space, operator} from '../../tokens';
import {DoWhileStatement, doWhileStatement, AnyNode} from '@romejs/js-ast';

export default function DoWhileStatement(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = doWhileStatement.assert(node);

  return [
    word('do'),
    space,
    ...generator.print(node.body, node),
    space,
    word('while'),
    space,
    operator('('),
    ...generator.print(node.test, node),
    operator(')'),
    operator(';'),
  ];
}
