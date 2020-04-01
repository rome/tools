/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, word, space, operator} from '../../tokens';
import {SwitchStatement, switchStatement, AnyNode} from '@romejs/js-ast';

export default function SwitchStatement(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = switchStatement.assert(node);

  return [
    word('switch'),
    space,
    operator('('),
    ...generator.print(node.discriminant, node),
    operator(')'),
    space,
    operator('{'),
    ...generator.printStatementList(node.cases, node, true),
    operator('}'),
  ];
}
