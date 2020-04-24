/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, word} from '../../tokens';
import {
  AnyNode,
  TSAssignmentAsExpression,
  tsAssignmentAsExpression,
} from '@romejs/js-ast';

export default function TSAssignmentAsExpression(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = tsAssignmentAsExpression.assert(node);

  if (builder.options.typeAnnotations) {
    return [
      ...builder.tokenize(node.expression, node),
      word('as'),
      ...builder.tokenize(node.typeAnnotation, node),
    ];
  } else {
    return builder.tokenize(node.expression, node);
  }
}
