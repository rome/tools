/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens} from '../../tokens';
import {
  ComputedMemberProperty,
  computedMemberProperty,
  AnyNode,
} from '@romejs/js-ast';
import {operator} from '@romejs/js-generator/tokens';

export default function ComputedMemberProperty(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = computedMemberProperty.assert(node);

  return [operator('['), ...generator.print(node.value, node), operator(']')];
}
