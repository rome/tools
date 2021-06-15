/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {descriptions} from "@internal/diagnostics";
import {createLintVisitor, signals} from "@internal/compiler";
import {jsBooleanLiteral, jsWhileStatement} from "@internal/ast";

export default createLintVisitor({
	name: "js/useWhile",
	enter(path) {
		const {node} = path;

		if (
			node.type === "JSForStatement" &&
			node.init === undefined &&
			node.update === undefined
		) {
			return path.addFixableDiagnostic(
				{
					fixed: signals.replace(
						jsWhileStatement.create(
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
					),
				},
				descriptions.LINT.JS_USE_WHILE,
			);
		}

		return signals.retain;
	},
});
