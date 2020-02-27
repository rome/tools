/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {NumericLiteral, numericLiteral, AnyNode} from '@romejs/js-ast';

export default function NumericLiteral(generator: Generator, node: AnyNode) {
  node =
    node.type === 'NumericLiteralTypeAnnotation'
      ? node
      : numericLiteral.assert(node);

  const value = String(node.value);
  generator.number(value);
}
