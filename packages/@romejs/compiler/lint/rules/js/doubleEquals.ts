/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from "@romejs/compiler";
import {AnyNode} from "@romejs/ast";
import {descriptions} from "@romejs/diagnostics";

const SUGGESTION_DESCRIPTION = "This may be unsafe if you are relying on type coercion";

export default {
	name: "doubleEquals",
	enter(path: Path): AnyNode {
		const {node, context} = path;

		if (
			node.type === "JSBinaryExpression" &&
			node.right.type !== "JSNullLiteral" &&
			node.left.type !== "JSNullLiteral"
		) {
			if (node.operator === "!=") {
				context.addFixableDiagnostic(
					{
						old: node,
						suggestions: [
							{
								title: "Use !==",
								description: SUGGESTION_DESCRIPTION,
								fixed: {
									...node,
									operator: "!==",
								},
							},
						],
					},
					descriptions.LINT.JS_NEGATE_DOUBLE_EQUALS,
				);
			}

			if (node.operator === "==") {
				context.addFixableDiagnostic(
					{
						old: node,
						suggestions: [
							{
								title: "Use ===",
								description: SUGGESTION_DESCRIPTION,
								fixed: {
									...node,
									operator: "===",
								},
							},
						],
					},
					descriptions.LINT.JS_DOUBLE_EQUALS,
				);
			}
		}

		return node;
	},
};
