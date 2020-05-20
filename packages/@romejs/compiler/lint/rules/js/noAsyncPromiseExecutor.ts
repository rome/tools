/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from "@romejs/compiler";
import {TransformExitResult} from "@romejs/compiler/types";
import {descriptions} from "@romejs/diagnostics";

export default {
	name: "noAsyncPromiseExecutor",
	enter(path: Path): TransformExitResult {
		const {node, context} = path;

		if (
			node.type === "JSNewExpression" &&
			node.callee.type === "JSReferenceIdentifier" &&
			node.callee.name === "Promise" &&
			node.arguments.length > 0 &&
			(node.arguments[0].type === "JSArrowFunctionExpression" ||
			node.arguments[0].type === "JSFunctionExpression") &&
			node.arguments[0].head.async
		) {
			context.addNodeDiagnostic(
				node.arguments[0],
				descriptions.LINT.JS_NO_ASYNC_PROMISE_EXECUTOR,
			);
		}

		return node;
	},
};
