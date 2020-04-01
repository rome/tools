/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  TSIndexedAccessType,
  tsIndexedAccessType,
  AnyNode,
} from '@romejs/js-ast';
import {Generator} from '@romejs/js-generator';
import {Tokens, operator} from '../../tokens';

export default function TSIndexedAccessType(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = tsIndexedAccessType.assert(node);

  return [
    ...generator.print(node.objectType, node),
    operator('['),
    ...generator.print(node.indexType, node),
    operator(']'),
  ];
}
