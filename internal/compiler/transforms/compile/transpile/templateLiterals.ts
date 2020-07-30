/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {createVisitor, signals} from "@internal/compiler";
import {
	AnyJSExpression,
	jsBinaryExpression,
	jsStringLiteral,
} from "@internal/ast";

export default createVisitor({
	name: "jsTemplateLiterals",
	enter(path) {
		const {node, parent} = path;

		if (node.type === "JSTaggedTemplateExpression") {
			// TODO
		}

		if (
			node.type === "JSTemplateLiteral" &&
			parent.type !== "JSTaggedTemplateExpression"
		) {
			const nodes: Array<AnyJSExpression> = [];
			const {expressions, quasis} = node;

			let index = 0;
			for (const elem of quasis) {
				if (elem.cooked) {
					nodes.push(
						jsStringLiteral.create({
							value: elem.cooked,
						}),
					);
				}

				if (index < expressions.length) {
					const expr = expressions[index++];
					if (expr.type !== "JSStringLiteral" || expr.value !== "") {
						nodes.push(expr);
					}
				}
			}

			if (nodes.length === 0) {
				return signals.replace(jsStringLiteral.quick(""));
			}

			if (nodes.length === 1) {
				return signals.replace(nodes[0]);
			}

			// Since `+` is left-to-right associative, nsure the first node is a string if first/second isn't
			if (
				nodes[0].type !== "JSStringLiteral" &&
				nodes[1].type !== "JSStringLiteral"
			) {
				nodes.unshift(jsStringLiteral.quick(""));
			}

			// Build the final expression
			let root = nodes[0];
			for (let i = 1; i < nodes.length; i++) {
				root = jsBinaryExpression.create({
					operator: "+",
					left: root,
					right: nodes[i],
				});
			}
			return signals.replace(root);
		}

		return signals.retain;
	},
});
