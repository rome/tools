/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {ComputedPropertyKey, computedPropertyKey, AnyNode} from '@romejs/js-ast';

export default function ComputedPropertyKey(generator: Generator, node: AnyNode) {
  node = computedPropertyKey.assert(node);

  generator.token('[');
  generator.print(node.value, node);
  generator.token(']');
}
