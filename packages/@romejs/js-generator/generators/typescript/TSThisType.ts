/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSThisType, tsThisType, AnyNode} from '@romejs/js-ast';
import {Generator} from '@romejs/js-generator';

export default function TSThisType(generator: Generator, node: AnyNode) {
  node = tsThisType.assert(node);
  tsThisType.assert(node);
  throw new Error('unimplemented');
}
