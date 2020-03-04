/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  TSImportEqualsDeclaration,
  tsImportEqualsDeclaration,
  AnyNode,
} from '@romejs/js-ast';
import {Generator} from '@romejs/js-generator';

export default function TSImportEqualsDeclaration(
  generator: Generator,
  node: AnyNode,
) {
  node = tsImportEqualsDeclaration.assert(node);

  if (node.isExport) {
    generator.word('export');
    generator.space();
  }

  generator.word('import');
  generator.space();
  generator.print(node.id, node);
  generator.space();
  generator.token('=');
  generator.space();
  generator.print(node.moduleReference, node);
  generator.token(';');
}
