/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {
  AnyNode,
  ExportExternalDeclaration,
  exportExternalDeclaration,
} from '@romejs/js-ast';
import {printModuleSpecifiers} from './ImportDeclaration';

export default function ExportExternalDeclaration(
  generator: Generator,
  node: AnyNode,
) {
  node = exportExternalDeclaration.assert(node);

  generator.word('export');
  generator.space();

  if (node.exportKind === 'type') {
    generator.word('type');
    generator.space();
  }

  generator.multiline(node, (multiline, node) => {
    printModuleSpecifiers(generator, node, multiline);
    generator.space();
    generator.word('from');
    generator.space();
    generator.print(node.source, node);

    generator.semicolon();
  });
}
