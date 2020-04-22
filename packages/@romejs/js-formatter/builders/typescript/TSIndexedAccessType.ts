/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  AnyNode,
  TSIndexedAccessType,
  tsIndexedAccessType,
} from '@romejs/js-ast';
import {Builder} from '@romejs/js-formatter';
import {Tokens, operator} from '../../tokens';

export default function TSIndexedAccessType(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = tsIndexedAccessType.assert(node);

  return [
    ...builder.tokenize(node.objectType, node),
    operator('['),
    ...builder.tokenize(node.indexType, node),
    operator(']'),
  ];
}
