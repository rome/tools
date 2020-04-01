/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSMethodSignature, tsMethodSignature, AnyNode} from '@romejs/js-ast';
import {Builder} from '@romejs/js-formatter';
import {Tokens, operator} from '../../tokens';

export default function TSMethodSignature(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = tsMethodSignature.assert(node);

  return [
    ...builder.print(node.key, node),
    ...builder.print(node.meta, node),
    operator(';'),
  ];
}
