/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSIndexSignature, tsIndexSignature, AnyNode} from '@romejs/js-ast';
import {Generator} from '@romejs/js-generator';
import {printBindingPatternParams} from '../utils';

export default function TSIndexSignature(generator: Generator, node: AnyNode) {
  node = tsIndexSignature.assert(node);

  if (node.readonly) {
    generator.word('readonly');
    generator.space();
  }

  generator.token('[');
  printBindingPatternParams(generator, node, node.parameters);
  generator.token(']');
  generator.print(node.typeAnnotation, node);
  generator.token(';');
}
