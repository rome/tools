/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path, TransformExitResult} from "@romejs/compiler";
import {jsUnaryExpression} from "@romejs/ast";
import {descriptions} from "@romejs/diagnostics";

export default {
	name: "unsafeNegation",
	enter(path: Path): TransformExitResult {
		const {node} = path;

		if (
			node.type === "JSBinaryExpression" &&
			(node.operator === "in" || node.operator === "instanceof") &&
			node.left.type === "JSUnaryExpression" &&
			node.left.operator === "!"
		) {
			return path.context.addFixableDiagnostic(
				{
					old: node,
					fixed: jsUnaryExpression.create({
						operator: node.left.operator,
						argument: {
							...node,
							left: node.left.argument,
						},
					}),
				},
				descriptions.LINT.JS_UNSAFE_NEGATION,
			);
		}

		return node;
	},
};
