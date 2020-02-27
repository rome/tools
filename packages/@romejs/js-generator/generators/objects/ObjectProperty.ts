/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {ObjectProperty, objectProperty, AnyNode} from '@romejs/js-ast';

export default function ObjectProperty(generator: Generator, node: AnyNode) {
  node = objectProperty.assert(node);

  generator.print(node.key, node);
  generator.token(':');
  generator.space();
  generator.print(node.value, node);
}
