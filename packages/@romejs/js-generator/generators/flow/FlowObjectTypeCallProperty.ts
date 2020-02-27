/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {
  FlowObjectTypeCallProperty,
  flowObjectTypeCallProperty,
  AnyNode,
} from '@romejs/js-ast';

export default function FlowObjectTypeCallProperty(
  generator: Generator,
  node: AnyNode,
) {
  node = flowObjectTypeCallProperty.assert(node);

  if (node.static === true) {
    generator.word('static');
    generator.space();
  }
  generator.print(node.value, node);
}
