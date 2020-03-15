/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {AnyNode, RegExpGroupCapture, regExpGroupCapture} from '@romejs/js-ast';

export default function RegExpGroupCapture(
  generator: Generator,
  node: AnyNode,
) {
  node = regExpGroupCapture.assert(node);

  generator.append('(');
  if (node.name !== undefined) {
    generator.append('?<');
    generator.append(node.name);
    generator.append('>');
  }
  generator.print(node.expression, node);
  generator.append(')');
}
