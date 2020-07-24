/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from "@romefrontend/compiler";
import {descriptions} from "@romefrontend/diagnostics";
import {markup} from "@romefrontend/cli-layout";

const SUGGESTION_DESCRIPTION = markup`This may be unsafe if you are relying on type coercion`;

export default {
	name: "js/doubleEquals",
	enter(path: Path) {
		const {node, context} = path;

		if (
			node.type === "JSBinaryExpression" &&
			node.right.type !== "JSNullLiteral" &&
			node.left.type !== "JSNullLiteral"
		) {
			if (node.operator === "!=") {
				return context.addFixableDiagnostic(
					{
						old: node,
						suggestions: [
							{
								title: markup`Use !==`,
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
				return context.addFixableDiagnostic(
					{
						old: node,
						suggestions: [
							{
								title: markup`Use ===`,
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
