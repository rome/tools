/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSConditionalType, tsConditionalType, AnyNode} from '@romejs/js-ast';
import {Generator} from '@romejs/js-generator';
import {Tokens, group, space, word, operator, newline} from '../../tokens';

export default function TSConditionalType(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = tsConditionalType.assert(node);

  return [
    ...generator.print(node.checkType, node),
    space,
    word('extends'),
    space,
    ...generator.print(node.extendsType, node),
    space,

    group([
      [operator('?'), space, ...generator.print(node.trueType, node)],
      [operator(':'), space, ...generator.print(node.falseType, node)],
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
