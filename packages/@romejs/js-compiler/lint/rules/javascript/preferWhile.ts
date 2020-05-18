/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {descriptions} from "@romejs/diagnostics";
import {Path, TransformExitResult} from "@romejs/js-compiler";
import {booleanLiteral, whileStatement} from "@romejs/js-ast";

export default {
	name: "preferWhile",
	enter(path: Path): TransformExitResult {
		const {context, node} = path;

		if (
			node.type === "ForStatement" &&
			node.init === undefined &&
			node.update === undefined
		) {
			return context.addFixableDiagnostic(
				{
					old: node,
					fixed: whileStatement.create(
						{
							test: node.test !== undefined
								? node.test
								: booleanLiteral.quick(true),
							body: node.body,
							leadingComments: node.leadingComments,
							trailingComments: node.trailingComments,
						},
						node,
					),
				},
				descriptions.LINT.JAVASCRIPT_PREFER_WHILE,
			);
		}

		return node;
	},
};
