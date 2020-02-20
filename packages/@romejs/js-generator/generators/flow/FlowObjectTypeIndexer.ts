/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {
  FlowObjectTypeIndexer,
  flowObjectTypeIndexer,
  AnyNode,
} from '@romejs/js-ast';

export default function FlowObjectTypeIndexer(
  generator: Generator,
  node: AnyNode,
) {
  node = flowObjectTypeIndexer.assert(node);

  if (node.static === true) {
    generator.word('static');
    generator.space();
  }

  generator.print(node.variance, node);
  generator.token('[');

  if (node.id !== undefined) {
    generator.print(node.id, node);
    generator.token(':');
  }

  generator.space();
  generator.print(node.key, node);
  generator.token(']');
  generator.token(':');
  generator.space();
  generator.print(node.value, node);
}
