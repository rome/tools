/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {
  IntersectionTypeAnnotation,
  intersectionTypeAnnotation,
  AnyNode,
} from '@romejs/js-ast';

export default function IntersectionTypeAnnotation(
  generator: Generator,
  node: AnyNode,
) {
  node = intersectionTypeAnnotation.assert(node);

  generator.printJoin(node.types, node, {separator: andSeparator});
}

function andSeparator(generator: Generator) {
  generator.space();
  generator.token('&');
  generator.space();
}
