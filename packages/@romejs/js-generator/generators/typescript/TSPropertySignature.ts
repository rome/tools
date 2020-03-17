/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  TSPropertySignature,
  tsPropertySignature,
  AnyNode,
} from '@romejs/js-ast';
import {Generator} from '@romejs/js-generator';

export default function TSPropertySignature(
  generator: Generator,
  node: AnyNode,
) {
  node = tsPropertySignature.assert(node);

  if (node.readonly) {
    generator.word('readonly');
    generator.space();
  }

  generator.print(node.key, node);

  if (node.optional) {
    generator.token('?');
  }

  generator.token(':');
  generator.space();

  generator.print(node.typeAnnotation, node);
  generator.token(';');
}
