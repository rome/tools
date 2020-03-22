/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSTypePredicate, tsTypePredicate, AnyNode} from '@romejs/js-ast';
import {Generator} from '@romejs/js-generator';

export default function TSTypePredicate(generator: Generator, node: AnyNode) {
  node = tsTypePredicate.assert(node);

  if (node.asserts) {
    generator.word('asserts');
    generator.space();
  }

  generator.print(node.parameterName, node);

  if (node.typeAnnotation) {
    generator.space();
    generator.word('is');
    generator.space();
    generator.print(node.typeAnnotation, node);
  }
}
