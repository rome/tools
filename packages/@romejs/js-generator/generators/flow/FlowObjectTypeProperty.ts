/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {
  FlowObjectTypeProperty,
  flowObjectTypeProperty,
  AnyNode,
} from '@romejs/js-ast';

export default function FlowObjectTypeProperty(
  generator: Generator,
  node: AnyNode,
) {
  node = flowObjectTypeProperty.assert(node);

  if (node.static === true) {
    generator.word('static');
    generator.space();
  }
  generator.print(node.variance, node);
  generator.print(node.key, node);
  if (node.optional === true) {
    generator.token('?');
  }
  generator.token(':');
  generator.space();
  generator.print(node.value, node);
}
