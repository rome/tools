/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSQualifiedName, tsQualifiedName, AnyNode} from '@romejs/js-ast';
import {Generator} from '@romejs/js-generator';

export default function TSQualifiedName(generator: Generator, node: AnyNode) {
  node = tsQualifiedName.assert(node);
  generator.print(node.left, node);
  generator.token('.');
  generator.print(node.right, node);
}
