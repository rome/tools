/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {
  ExportDefaultDeclaration,
  exportDefaultDeclaration,
  AnyNode,
} from '@romejs/js-ast';
import {_ExportDeclaration} from './ExportNamedDeclaration';
export default function ExportDefaultDeclaration(
  generator: Generator,
  node: AnyNode,
) {
  node = exportDefaultDeclaration.assert(node);

  generator.word('export');
  generator.space();
  generator.word('default');
  generator.space();
  _ExportDeclaration(generator, node);
}
