/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {ImportSpecifier, importSpecifier, AnyNode} from '@romejs/js-ast';

export default function ImportSpecifier(generator: Generator, node: AnyNode) {
  node = importSpecifier.assert(node);

  if (node.local.importKind === 'type' || node.local.importKind === 'typeof') {
    generator.word(node.local.importKind);
    generator.space();
  }

  generator.print(node.imported, node);

  if (node.local.name.name !== node.imported.name) {
    generator.space();
    generator.word('as');
    generator.space();
    generator.print(node.local.name, node);
  }
}
