/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  AnyNode,
  TSTypeParameterInstantiation,
  tsTypeParameterInstantiation,
} from '@romejs/js-ast';
import {Generator} from '@romejs/js-generator';

export default function TSTypeParameterInstantiation(
  generator: Generator,
  node: AnyNode,
) {
  node = tsTypeParameterInstantiation.assert(node);
  generator.token('<');
  generator.printCommaList(node.params, node);
  generator.token('>');
}
