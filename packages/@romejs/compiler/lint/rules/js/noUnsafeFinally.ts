/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from "@romejs/compiler";
import {AnyNode} from "@romejs/ast";
import {descriptions} from "@romejs/diagnostics";

export default {
	name: "noUnsafeFinally",
	enter(path: Path): AnyNode {
		const {node, context} = path;

		if (node.type === "JSTryStatement") {
			const {finalizer} = node;

			if (finalizer && finalizer.type === "JSBlockStatement") {
				for (const statement of finalizer.body) {
					if (
						statement.type === "JSThrowStatement" ||
						statement.type === "JSContinueStatement" ||
						statement.type === "JSBreakStatement" ||
						statement.type === "JSReturnStatement"
					) {
						context.addNodeDiagnostic(
							statement,
							descriptions.LINT.JS_NO_UNSAFE_FINALLY(statement.type),
						);
					}
				}
			}
		}

		return node;
	},
};
