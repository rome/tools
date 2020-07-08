/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from "@romefrontend/compiler";
import {AnyNode} from "@romefrontend/ast";
import {descriptions} from "@romefrontend/diagnostics";

export default {
	name: "noCondAssign",
	enter(path: Path): AnyNode {
		const {node} = path;

		if (
			(node.type === "JSIfStatement" ||
			node.type === "JSForStatement" ||
			node.type === "JSWhileStatement" ||
			node.type === "JSDoWhileStatement" ||
			node.type === "JSConditionalExpression") &&
			node.test &&
			node.test.type === "JSAssignmentExpression"
		) {
			path.context.addNodeDiagnostic(
				node.test,
				descriptions.LINT.JS_NO_COND_ASSIGN,
			);
		}

		return node;
	},
};
