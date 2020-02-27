/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {
  FlowTupleTypeAnnotation,
  flowTupleTypeAnnotation,
  AnyNode,
} from '@romejs/js-ast';

export default function FlowTupleTypeAnnotation(
  generator: Generator,
  node: AnyNode,
) {
  node = flowTupleTypeAnnotation.assert(node);

  generator.token('[');
  generator.printCommaList(node.types, node);
  generator.token(']');
}
