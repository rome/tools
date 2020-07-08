/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {descriptions} from "@romefrontend/diagnostics";
import {Path, TransformExitResult} from "@romefrontend/compiler";
import {jsBooleanLiteral, jsWhileStatement} from "@romefrontend/ast";

export default {
	name: "preferWhile",
	enter(path: Path): TransformExitResult {
		const {context, node} = path;

		if (
			node.type === "JSForStatement" &&
			node.init === undefined &&
			node.update === undefined
		) {
			return context.addFixableDiagnostic(
				{
					old: node,
					fixed: jsWhileStatement.create(
						{
							test: node.test !== undefined
								? node.test
								: jsBooleanLiteral.quick(true),
							body: node.body,
							leadingComments: node.leadingComments,
							trailingComments: node.trailingComments,
						},
						node,
					),
				},
				descriptions.LINT.JS_PREFER_WHILE,
			);
		}

		return node;
	},
};
