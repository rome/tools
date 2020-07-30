/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode, JSBinaryExpression, JSLogicalExpression} from "@internal/ast";

export function isBinary(
	node: undefined | AnyNode,
): node is JSBinaryExpression | JSLogicalExpression {
	if (node === undefined) {
		return false;
	}

	switch (node.type) {
		case "JSBinaryExpression":
		case "JSLogicalExpression":
			return true;

		default:
			return false;
	}
}
