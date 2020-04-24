/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode, TSConditionalType, tsConditionalType} from '@romejs/js-ast';
import {Builder} from '@romejs/js-formatter';
import {Tokens, group, newline, operator, space, word} from '../../tokens';

export default function TSConditionalType(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = tsConditionalType.assert(node);

  return [
    ...builder.tokenize(node.checkType, node),
    space,
    word('extends'),
    space,
    ...builder.tokenize(node.extendsType, node),
    space,

    group([
      [operator('?'), space, ...builder.tokenize(node.trueType, node)],
      [operator(':'), space, ...builder.tokenize(node.falseType, node)],
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
