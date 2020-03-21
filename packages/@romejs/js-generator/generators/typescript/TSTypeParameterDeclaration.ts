/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  TSTypeParameterDeclaration,
  tsTypeParameterDeclaration,
  AnyNode,
} from '@romejs/js-ast';
import {Generator} from '@romejs/js-generator';

export default function TSTypeParameterDeclaration(
  generator: Generator,
  node: AnyNode,
) {
  node = tsTypeParameterDeclaration.assert(node);
  generator.token('<');
  generator.printCommaList(node.params, node);
  generator.token('>');
}
