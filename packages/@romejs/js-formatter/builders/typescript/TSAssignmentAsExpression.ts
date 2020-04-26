/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSAssignmentAsExpression} from '@romejs/js-ast';
import Builder from '../../Builder';
import {Token, concat, space} from '../../tokens';

export default function TSAssignmentAsExpression(
  builder: Builder,
  node: TSAssignmentAsExpression,
): Token {
  if (builder.options.typeAnnotations) {
    return concat([
      builder.tokenize(node.expression, node),
      space,
      'as',
      space,
      builder.tokenize(node.typeAnnotation, node),
    ]);
  } else {
    return builder.tokenize(node.expression, node);
  }
}
