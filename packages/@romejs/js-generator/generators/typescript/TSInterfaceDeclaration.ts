/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  TSInterfaceDeclaration,
  tsInterfaceDeclaration,
  AnyNode,
} from '@romejs/js-ast';
import {Generator} from '@romejs/js-generator';

export default function TSInterfaceDeclaration(
  generator: Generator,
  node: AnyNode,
) {
  node = tsInterfaceDeclaration.assert(node);

  if (node.declare) {
    generator.word('declare');
    generator.space();
  }

  generator.word('interface');
  generator.space();
  generator.print(node.id, node);
  generator.print(node.typeParameters, node);

  if (node.extends) {
    generator.space();
    generator.word('extends');
    generator.space();
    generator.printCommaList(node.extends, node);
  }

  generator.space();
  generator.print(node.body, node);
}
