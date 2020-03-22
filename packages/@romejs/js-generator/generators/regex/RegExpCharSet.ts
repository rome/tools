/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {AnyNode, RegExpCharSet, regExpCharSet} from '@romejs/js-ast';

export default function RegExpCharSet(generator: Generator, node: AnyNode) {
  node = regExpCharSet.assert(node);
  generator.append('[');
  if (node.invert) {
    generator.append('^');
  }
  for (const item of node.body) {
    generator.print(item, node);
  }
  generator.append(']');
}
