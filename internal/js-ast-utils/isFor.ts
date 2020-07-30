/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyNode,
	JSForInStatement,
	JSForOfStatement,
	JSForStatement,
} from "@internal/ast";

export function isFor(
	node: undefined | AnyNode,
): node is JSForStatement | JSForInStatement | JSForOfStatement {
	if (node === undefined) {
		return false;
	}

	switch (node.type) {
		case "JSForStatement":
		case "JSForInStatement":
		case "JSForOfStatement":
			return true;

		default:
			return false;
	}
}
