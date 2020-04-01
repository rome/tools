/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, operator} from '../../tokens';
import {
  ComputedPropertyKey,
  computedPropertyKey,
  AnyNode,
} from '@romejs/js-ast';

export default function ComputedPropertyKey(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = computedPropertyKey.assert(node);
  return [operator('['), ...builder.print(node.value, node), operator(']')];
}
