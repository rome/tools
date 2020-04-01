/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, space, word, operator} from '../../tokens';
import {UnaryExpression, unaryExpression, AnyNode} from '@romejs/js-ast';

export default function UnaryExpression(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = unaryExpression.assert(node);

  if (
    node.operator === 'void' ||
    node.operator === 'delete' ||
    node.operator === 'typeof'
  ) {
    return [
      word(node.operator),
      space,
      ...generator.print(node.argument, node),
    ];
  } else {
    return [operator(node.operator), ...generator.print(node.argument, node)];
  }
}
