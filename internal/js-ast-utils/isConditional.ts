/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode, JSConditionalExpression, JSIfStatement} from "@internal/ast";

export function isConditional(
	node: undefined | AnyNode,
): node is JSConditionalExpression | JSIfStatement {
	if (node === undefined) {
		return false;
	}

	switch (node.type) {
		case "JSConditionalExpression":
		case "JSIfStatement":
			return true;

		case "JSLogicalExpression":
			return true;

		default:
			return false;
	}
}
