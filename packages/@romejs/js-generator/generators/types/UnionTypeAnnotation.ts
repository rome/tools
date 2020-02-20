/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {
  UnionTypeAnnotation,
  unionTypeAnnotation,
  AnyNode,
} from '@romejs/js-ast';

export default function UnionTypeAnnotation(
  generator: Generator,
  node: AnyNode,
) {
  node = unionTypeAnnotation.assert(node);

  generator.printJoin(node.types, node, {separator: orSeparator});
}

function orSeparator(generator: Generator) {
  generator.space();
  generator.token('|');
  generator.space();
}
