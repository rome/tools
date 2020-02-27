/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {SpreadProperty, spreadProperty, AnyNode} from '@romejs/js-ast';

export default function SpreadProperty(generator: Generator, node: AnyNode) {
  node = spreadProperty.assert(node);

  generator.token('...');
  generator.print(node.argument, node);
}
