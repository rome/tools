/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  TSConstructSignatureDeclaration,
  tsConstructSignatureDeclaration,
  AnyNode,
} from '@romejs/js-ast';
import {Generator} from '@romejs/js-generator';

export default function TSConstructSignatureDeclaration(
  generator: Generator,
  node: AnyNode,
) {
  node = tsConstructSignatureDeclaration.assert(node);

  generator.word('new');
  generator.space();
  generator.print(node.meta, node);

  if (node.typeAnnotation !== undefined) {
    generator.token(':');
    generator.space();
    generator.print(node.typeAnnotation, node);
  }

  generator.token(';');
}
