/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  TSExternalModuleReference,
  tsExternalModuleReference,
  AnyNode,
} from '@romejs/js-ast';
import {Generator} from '@romejs/js-generator';

export default function TSExternalModuleReference(
  generator: Generator,
  node: AnyNode,
) {
  node = tsExternalModuleReference.assert(node);
  generator.token('require(');
  generator.print(node.expression, node);
  generator.token(')');
}
