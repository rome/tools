/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {
  NumericLiteralTypeAnnotation,
  numericLiteralTypeAnnotation,
  AnyNode,
} from '@romejs/js-ast';

export default function NumericLiteralTypeAnnotation(
  generator: Generator,
  node: AnyNode,
) {
  node = numericLiteralTypeAnnotation.assert(node);

  numericLiteralTypeAnnotation.assert(node);
  throw new Error('unimplemented');
}
