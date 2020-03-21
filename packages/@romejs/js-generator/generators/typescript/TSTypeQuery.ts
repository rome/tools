/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSTypeQuery, tsTypeQuery, AnyNode} from '@romejs/js-ast';
import {Generator} from '@romejs/js-generator';

export default function TSTypeQuery(generator: Generator, node: AnyNode) {
  node = tsTypeQuery.assert(node);
  generator.word('typeof');
  generator.space();
  generator.print(node.exprName, node);
}
