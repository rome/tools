/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {createLintVisitor, signals} from "@internal/compiler";
import {jsUnaryExpression} from "@internal/ast";
import {descriptions} from "@internal/diagnostics";

export default createLintVisitor({
	name: "js/noUnsafeNegation",
	enter(path) {
		const {node} = path;

		if (
			node.type === "JSBinaryExpression" &&
			(node.operator === "in" || node.operator === "instanceof") &&
			node.left.type === "JSUnaryExpression" &&
			node.left.operator === "!"
		) {
			return path.addFixableDiagnostic(
				{
					fixed: signals.replace(
						jsUnaryExpression.create({
							operator: node.left.operator,
							argument: {
								...node,
								left: node.left.argument,
							},
						}),
					),
				},
				descriptions.LINT.JS_UNSAFE_NEGATION,
			);
		}

		return signals.retain;
	},
});
