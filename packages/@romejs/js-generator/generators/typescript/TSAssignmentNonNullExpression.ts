/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {
  TSAssignmentNonNullExpression,
  tsAssignmentNonNullExpression,
  AnyNode,
} from '@romejs/js-ast';

export default function TSAssignmentNonNullExpression(
  generator: Generator,
  node: AnyNode,
) {
  node = tsAssignmentNonNullExpression.assert(node);
  generator.print(node.expression, node);
  generator.token('!');
}
