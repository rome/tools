/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, operator, space, group, newline} from '../../tokens';
import {
  ConditionalExpression,
  conditionalExpression,
  AnyNode,
} from '@romejs/js-ast';

export default function ConditionalExpression(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = conditionalExpression.assert(node);

  return [
    ...generator.print(node.test, node),
    space,
    group(
      [
        [operator('?'), space, ...generator.print(node.consequent, node)],
        [operator(':'), space, ...generator.print(node.alternate, node)],
      ],
      {
        priority: true,
        broken: {
          before: [newline],
          indentNewline: false,
          separator: [newline],
        },
        unbroken: {
          separator: [space],
        },
      },
    ),
  ];
}
