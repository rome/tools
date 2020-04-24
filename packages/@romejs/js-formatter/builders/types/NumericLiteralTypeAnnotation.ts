/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, number} from '../../tokens';
import {
  AnyNode,
  NumericLiteralTypeAnnotation,
  numericLiteralTypeAnnotation,
} from '@romejs/js-ast';

export default function NumericLiteralTypeAnnotation(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = numericLiteralTypeAnnotation.assert(node);
  return [number(String(node.value))];
}
