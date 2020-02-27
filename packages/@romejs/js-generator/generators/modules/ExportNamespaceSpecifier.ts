/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {
  ExportNamespaceSpecifier,
  exportNamespaceSpecifier,
  AnyNode,
} from '@romejs/js-ast';

export default function ExportNamespaceSpecifier(
  generator: Generator,
  node: AnyNode,
) {
  node = exportNamespaceSpecifier.assert(node);

  generator.token('*');
  generator.space();
  generator.word('as');
  generator.space();
  generator.print(node.exported, node);
}
