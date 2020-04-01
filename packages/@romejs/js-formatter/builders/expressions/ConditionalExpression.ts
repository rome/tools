/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, operator, space, group, newline} from '../../tokens';
import {
  ConditionalExpression,
  conditionalExpression,
  AnyNode,
} from '@romejs/js-ast';

export default function ConditionalExpression(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = conditionalExpression.assert(node);

  return [
    ...builder.tokenize(node.test, node),
    space,
    group(
      [
        [operator('?'), space, ...builder.tokenize(node.consequent, node)],
        [operator(':'), space, ...builder.tokenize(node.alternate, node)],
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
