/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {AnyNode, classPrivateProperty} from '@romejs/js-ast';

export default function ClassPrivateProperty(generator: Generator, node: AnyNode) {
  node = classPrivateProperty.assert(node);

  generator.print(node.meta, node);
  generator.print(node.key, node);
  generator.printTypeColon(node.typeAnnotation, node);

  if (node.value) {
    generator.space();
    generator.token('=');
    generator.space();
    generator.print(node.value, node);
  }

  generator.semicolon();
}
