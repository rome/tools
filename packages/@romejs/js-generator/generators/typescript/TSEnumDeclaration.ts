/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSEnumDeclaration, tsEnumDeclaration, AnyNode} from '@romejs/js-ast';
import {Generator} from '@romejs/js-generator';
import {printTSBraced} from '../utils';

export default function TSEnumDeclaration(generator: Generator, node: AnyNode) {
  node = tsEnumDeclaration.assert(node);

  if (node.declare) {
    generator.word('declare');
    generator.space();
  }

  if (node.const) {
    generator.word('const');
    generator.space();
  }

  generator.word('enum');
  generator.space();
  generator.print(node.id, node);
  generator.space();
  printTSBraced(generator, node, node.members);
}
