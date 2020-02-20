/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  TSAssignmentNonNullExpression,
  tsAssignmentNonNullExpression,
  AnyNode,
} from '@romejs/js-ast';

export default function TSAssignmentNonNullExpression(node: AnyNode) {
  node = tsAssignmentNonNullExpression.assert(node);
  throw new Error('unimplemented');
}
