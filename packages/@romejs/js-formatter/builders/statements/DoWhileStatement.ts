/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, word, space, operator, concat} from '../../tokens';
import {doWhileStatement, AnyNode} from '@romejs/js-ast';

export default function DoWhileStatement(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = doWhileStatement.assert(node);

  return [
    word('do'),
    space,
    concat(builder.tokenize(node.body, node)),
    space,
    word('while'),
    space,
    operator('('),
    concat(builder.tokenize(node.test, node)),
    operator(')'),
    operator(';'),
  ];
}
