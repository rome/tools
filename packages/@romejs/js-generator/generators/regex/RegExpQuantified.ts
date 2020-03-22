/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {AnyNode, RegExpQuantified, regExpQuantified} from '@romejs/js-ast';

export default function RegExpQuantified(generator: Generator, node: AnyNode) {
  node = regExpQuantified.assert(node);
  generator.print(node.target, node);

  if (node.min === 0 && node.max === 1) {
    generator.append('?');
  } else if (node.min === 0 && node.max === undefined) {
    generator.append('*');
  } else if (node.min === 1 && node.max === undefined) {
    generator.append('+');
  } else {
    generator.append('{');

    generator.append(String(node.min));

    if (node.min !== node.max) {
      generator.token(',');
      if (node.max !== undefined) {
        generator.append(String(node.max));
      }
    }

    generator.append('}');
  }

  if (node.lazy) {
    generator.append('?');
  }
}
