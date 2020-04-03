/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, word, space, operator} from '../../tokens';
import {WithStatement, withStatement, AnyNode} from '@romejs/js-ast';

export default function WithStatement(builder: Builder, node: AnyNode): Tokens {
  node = withStatement.assert(node);

  return [
    word('with'),
    space,
    operator('('),
    ...builder.tokenize(node.object, node),
    operator(')'),
    ...builder.tokenize(node.body, node),
  ];
}
