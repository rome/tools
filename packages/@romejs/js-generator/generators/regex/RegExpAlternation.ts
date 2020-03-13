/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {AnyNode, RegExpAlternation, regExpAlternation} from '@romejs/js-ast';

export default function RegExpAlternation(generator: Generator, node: AnyNode) {
  node = regExpAlternation.assert(node);
  generator.print(node.left, node);
  generator.token('|');
  generator.print(node.right, node);
}
