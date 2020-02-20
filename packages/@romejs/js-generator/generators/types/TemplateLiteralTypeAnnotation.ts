/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {
  TemplateLiteralTypeAnnotation,
  templateLiteralTypeAnnotation,
  AnyNode,
} from '@romejs/js-ast';

export default function TemplateLiteralTypeAnnotation(
  generator: Generator,
  node: AnyNode,
) {
  node = templateLiteralTypeAnnotation.assert(node);

  templateLiteralTypeAnnotation.assert(node);
  throw new Error('unimplemented');
}
