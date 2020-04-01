/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, word} from '../../tokens';
import {
  TSAssignmentAsExpression,
  tsAssignmentAsExpression,
  AnyNode,
} from '@romejs/js-ast';

export default function TSAssignmentAsExpression(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = tsAssignmentAsExpression.assert(node);

  if (generator.options.typeAnnotations) {
    return [
      ...generator.print(node.expression, node),
      word('as'),
      ...generator.print(node.typeAnnotation, node),
    ];
  } else {
    return generator.print(node.expression, node);
  }
}
