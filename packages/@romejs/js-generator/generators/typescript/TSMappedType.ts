/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSMappedType, tsMappedType, AnyNode} from '@romejs/js-ast';
import {Generator} from '@romejs/js-generator';

export default function TSMappedType(generator: Generator, node: AnyNode) {
  node = tsMappedType.assert(node);
  tsMappedType.assert(node);
  throw new Error('unimplemented');
}
