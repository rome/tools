/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, word} from '../../tokens';
import {
  TSAssignmentAsExpression,
  tsAssignmentAsExpression,
  AnyNode,
} from '@romejs/js-ast';

export default function TSAssignmentAsExpression(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = tsAssignmentAsExpression.assert(node);

  if (builder.options.typeAnnotations) {
    return [
      ...builder.print(node.expression, node),
      word('as'),
      ...builder.print(node.typeAnnotation, node),
    ];
  } else {
    return builder.print(node.expression, node);
  }
}
