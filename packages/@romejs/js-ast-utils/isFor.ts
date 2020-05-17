/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyNode,
	ForInStatement,
	ForOfStatement,
	ForStatement,
} from "@romejs/js-ast";

export default function isFor(
	node: undefined | AnyNode,
): node is ForStatement | ForInStatement | ForOfStatement {
	if (node === undefined) {
		return false;
	}

	switch (node.type) {
		case "ForStatement":
		case "ForInStatement":
		case "ForOfStatement":
			return true;

		default:
			return false;
	}
}
