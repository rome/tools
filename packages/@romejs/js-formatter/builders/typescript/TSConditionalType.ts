/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSConditionalType, tsConditionalType, AnyNode} from '@romejs/js-ast';
import {Builder} from '@romejs/js-formatter';
import {Tokens, group, space, word, operator, newline} from '../../tokens';

export default function TSConditionalType(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = tsConditionalType.assert(node);

  return [
    ...builder.print(node.checkType, node),
    space,
    word('extends'),
    space,
    ...builder.print(node.extendsType, node),
    space,

    group([
      [operator('?'), space, ...builder.print(node.trueType, node)],
      [operator(':'), space, ...builder.print(node.falseType, node)],
    ], {
      priority: true,
      broken: {
        separator: [newline],
      },
      unbroken: {
        separator: [space],
      },
    }),
  ];
}
