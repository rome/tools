/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens} from '../../tokens';
import {
  AnyNode,
  StringLiteralTypeAnnotation,
  stringLiteralTypeAnnotation,
} from '@romejs/js-ast';
import StringLiteral from '../literals/StringLiteral';

export default function StringLiteralTypeAnnotation(
  builder: Builder,
  node: AnyNode,
  parent: AnyNode,
): Tokens {
  node = stringLiteralTypeAnnotation.assert(node);
  return StringLiteral(builder, node, parent);
}
