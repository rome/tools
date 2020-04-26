/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSNonNullExpression} from '@romejs/js-ast';
import {Builder} from '@romejs/js-formatter';
import {Token, concat} from '../../tokens';

export default function TSNonNullExpression(
  builder: Builder,
  node: TSNonNullExpression,
): Token {
  const expr = builder.tokenize(node.expression, node);

  if (builder.options.typeAnnotations) {
    return concat([expr, '!']);
  } else {
    return expr;
  }
}
