/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  TSNonNullExpression,
  tsNonNullExpression,
  AnyNode,
} from '@romejs/js-ast';
import {Generator} from '@romejs/js-generator';
import {Tokens, operator} from '../../tokens';

export default function TSNonNullExpression(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = tsNonNullExpression.assert(node);

  if (generator.options.typeAnnotations) {
    return [...generator.print(node.expression, node), operator('!')];
  } else {
    return generator.print(node.expression, node);
  }
}
