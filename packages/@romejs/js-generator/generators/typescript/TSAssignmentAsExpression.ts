/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {
  TSAssignmentAsExpression,
  tsAssignmentAsExpression,
  AnyNode,
} from '@romejs/js-ast';

export default function TSAssignmentAsExpression(
  generator: Generator,
  node: AnyNode,
) {
  node = tsAssignmentAsExpression.assert(node);

  generator.print(node.expression);

  if (generator.options.typeAnnotations) {
    generator.word('as');
    generator.print(node.typeAnnotation, node);
  }
}
