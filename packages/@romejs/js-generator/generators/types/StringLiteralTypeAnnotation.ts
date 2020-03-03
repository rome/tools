/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {
  StringLiteralTypeAnnotation,
  stringLiteralTypeAnnotation,
  AnyNode,
} from '@romejs/js-ast';
import StringLiteral from '../literals/StringLiteral';

export default function StringLiteralTypeAnnotation(
  generator: Generator,
  node: AnyNode,
  parent: AnyNode,
) {
  node = stringLiteralTypeAnnotation.assert(node);
  StringLiteral(generator, node, parent);
}
