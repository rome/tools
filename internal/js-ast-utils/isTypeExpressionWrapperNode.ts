/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyNode,
	TSAsExpression,
	TSNonNullExpression,
	TSTypeAssertion,
} from "@internal/ast";

export function isTypeExpressionWrapperNode(
	node: AnyNode,
): node is TSAsExpression | TSTypeAssertion | TSNonNullExpression {
	return (
		node.type === "TSAsExpression" ||
		node.type === "TSTypeAssertion" ||
		node.type === "TSNonNullExpression"
	);
}
