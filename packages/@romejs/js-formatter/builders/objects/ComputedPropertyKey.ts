/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, operator, concat} from '../../tokens';
import {computedPropertyKey, AnyNode} from '@romejs/js-ast';

export default function ComputedPropertyKey(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = computedPropertyKey.assert(node);
  return [
    operator('['),
    concat(builder.tokenize(node.value, node)),
    operator(']'),
  ];
}
