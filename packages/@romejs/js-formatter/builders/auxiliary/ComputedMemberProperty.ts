/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens} from '../../tokens';
import {
  ComputedMemberProperty,
  computedMemberProperty,
  AnyNode,
} from '@romejs/js-ast';
import {operator} from '@romejs/js-formatter/tokens';

export default function ComputedMemberProperty(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = computedMemberProperty.assert(node);

  return [operator('['), ...builder.print(node.value, node), operator(']')];
}
