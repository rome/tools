/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  AnyNode,
  FlowTypeCastExpression,
  TSAsExpression,
  TSTypeAssertion,
  TSNonNullExpression,
} from '@romejs/js-ast';

export default function isTypeExpressionWrapperNode(
  node: AnyNode,
): node is
  | FlowTypeCastExpression
  | TSAsExpression
  | TSTypeAssertion
  | TSNonNullExpression {
  return node.type === 'FlowTypeCastExpression' || node.type ===
    'TSAsExpression' || node.type === 'TSTypeAssertion' || node.type ===
    'TSNonNullExpression';
}
