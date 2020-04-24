/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, concat, operator, space, word} from '../../tokens';
import {AnyNode, withStatement} from '@romejs/js-ast';

export default function WithStatement(builder: Builder, node: AnyNode): Tokens {
  node = withStatement.assert(node);

  return [
    word('with'),
    space,
    operator('('),
    concat(builder.tokenize(node.object, node)),
    operator(')'),
    concat(builder.tokenize(node.body, node)),
  ];
}
