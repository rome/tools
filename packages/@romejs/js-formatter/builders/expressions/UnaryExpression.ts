/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, concat, operator, space, word} from '../../tokens';
import {AnyNode, unaryExpression} from '@romejs/js-ast';

export default function UnaryExpression(builder: Builder, node: AnyNode): Tokens {
  node = unaryExpression.assert(node);

  if (node.operator === 'void' || node.operator === 'delete' ||
        node.operator ===
        'typeof') {
    return [
      word(node.operator),
      space,
      concat(builder.tokenize(node.argument, node)),
    ];
  } else {
    return [
      operator(node.operator),
      concat(builder.tokenize(node.argument, node)),
    ];
  }
}
