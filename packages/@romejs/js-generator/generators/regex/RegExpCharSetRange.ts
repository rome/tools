/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {AnyNode, RegExpCharSetRange, regExpCharSetRange} from '@romejs/js-ast';

export default function RegExpCharSetRange(
  generator: Generator,
  node: AnyNode,
) {
  node = regExpCharSetRange.assert(node);
  generator.print(node.start, node);
  generator.append('-');
  generator.print(node.end, node);
}
