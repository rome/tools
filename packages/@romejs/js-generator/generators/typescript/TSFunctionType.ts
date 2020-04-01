/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSFunctionType, tsFunctionType, AnyNode} from '@romejs/js-ast';
import {Generator} from '@romejs/js-generator';
import {Tokens, operator, space, linkedGroups} from '../../tokens';

export default function TSFunctionType(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = tsFunctionType.assert(node);

  return [
    linkedGroups([
      ...generator.print(node.meta, node),
      space,
      operator('=>'),
      space,
      ...generator.print(node.typeAnnotation, node),
    ]),
  ];
}
