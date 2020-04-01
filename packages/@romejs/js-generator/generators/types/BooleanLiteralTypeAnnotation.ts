/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, word} from '../../tokens';
import {
  BooleanLiteralTypeAnnotation,
  booleanLiteralTypeAnnotation,
  AnyNode,
} from '@romejs/js-ast';

export default function BooleanLiteralTypeAnnotation(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = booleanLiteralTypeAnnotation.assert(node);
  return [word(node.value ? 'true' : 'false')];
}
