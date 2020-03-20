/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSMethodSignature, tsMethodSignature, AnyNode} from '@romejs/js-ast';
import {Generator} from '@romejs/js-generator';

export default function TSMethodSignature(generator: Generator, node: AnyNode) {
  node = tsMethodSignature.assert(node);
  generator.print(node.key, node);
  generator.print(node.meta, node);
  generator.token(';');
}
