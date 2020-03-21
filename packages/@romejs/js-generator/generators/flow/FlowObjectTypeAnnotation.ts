/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {
  FlowObjectTypeAnnotation,
  flowObjectTypeAnnotation,
  AnyNode,
} from '@romejs/js-ast';

export default function FlowObjectTypeAnnotation(
  generator: Generator,
  node: AnyNode,
) {
  node = flowObjectTypeAnnotation.assert(node);

  if (node.exact === true) {
    generator.token('{|');
  } else {
    generator.token('{');
  }

  generator.printCommaList(node.properties, node);

  if (node.exact === true) {
    generator.token('|}');
  } else {
    generator.token('}');
  }
}
