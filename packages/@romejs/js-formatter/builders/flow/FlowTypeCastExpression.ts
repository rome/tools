/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Token, concat, space} from '../../tokens';
import {FlowTypeCastExpression} from '@romejs/js-ast';

export default function FlowTypeCastExpression(
  builder: Builder,
  node: FlowTypeCastExpression,
): Token {
  if (builder.options.typeAnnotations) {
    return concat([
      '(',
      builder.tokenize(node.expression, node),
      ':',
      space,
      builder.tokenize(node.typeAnnotation, node),
      ')',
    ]);
  } else {
    return builder.tokenize(node.expression, node);
  }
}
