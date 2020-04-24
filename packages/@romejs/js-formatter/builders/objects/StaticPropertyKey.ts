/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens} from '../../tokens';
import {AnyNode, StaticPropertyKey, staticPropertyKey} from '@romejs/js-ast';

export default function StaticPropertyKey(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = staticPropertyKey.assert(node);
  return builder.tokenize(node.value, node);
}
