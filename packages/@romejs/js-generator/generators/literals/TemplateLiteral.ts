/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {TemplateLiteral, templateLiteral, AnyNode} from '@romejs/js-ast';

export default function TemplateLiteral(generator: Generator, node: AnyNode) {
  node = templateLiteral.assert(node);

  const quasis = node.quasis;

  for (let i = 0;
  i < quasis.length;
  i++) {
    generator.print(quasis[i], node);

    if (i + 1 < quasis.length) {
      generator.print(node.expressions[i], node);
    }
  }
}
