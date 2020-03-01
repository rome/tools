/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSExportAssignment, tsExportAssignment, AnyNode} from '@romejs/js-ast';
import {Generator} from '@romejs/js-generator';

export default function TSExportAssignment(
  generator: Generator,
  node: AnyNode,
) {
  node = tsExportAssignment.assert(node);
  generator.word('export');
  generator.space();
  generator.token('=');
  generator.space();
  generator.print(node.expression, node);
  generator.token(';');
}
