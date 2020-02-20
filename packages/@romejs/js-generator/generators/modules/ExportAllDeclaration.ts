/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {
  ExportAllDeclaration,
  exportAllDeclaration,
  AnyNode,
} from '@romejs/js-ast';

export default function ExportAllDeclaration(
  generator: Generator,
  node: AnyNode,
) {
  node = exportAllDeclaration.assert(node);

  generator.word('export');
  generator.space();
  generator.token('*');
  generator.space();
  generator.word('from');
  generator.space();
  generator.print(node.source, node);
  generator.semicolon();
}
