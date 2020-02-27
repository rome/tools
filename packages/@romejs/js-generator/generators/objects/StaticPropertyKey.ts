/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {StaticPropertyKey, staticPropertyKey, AnyNode} from '@romejs/js-ast';

export default function StaticPropertyKey(generator: Generator, node: AnyNode) {
  node = staticPropertyKey.assert(node);

  generator.print(node.value, node);
}
