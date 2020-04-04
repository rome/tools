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
import {Builder} from '@romejs/js-formatter';
import {Tokens, operator} from '../../tokens';

export default function TSNonNullExpression(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = tsNonNullExpression.assert(node);

  if (builder.options.typeAnnotations) {
    return [...builder.tokenize(node.expression, node), operator('!')];
  } else {
    return builder.tokenize(node.expression, node);
  }
}
