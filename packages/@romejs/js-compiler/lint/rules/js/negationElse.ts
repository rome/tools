/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from "@romejs/js-compiler";
import {AnyExpression, UnaryExpression} from "@romejs/js-ast";
import {descriptions} from "@romejs/diagnostics";

function isNegation(node: AnyExpression): node is UnaryExpression {
	return (
		node.type === "UnaryExpression" &&
		node.prefix === true &&
		node.operator === "!"
	);
}

export default {
	name: "negationElse",
	enter(path: Path) {
		const {node} = path;

		if (
			node.type === "IfStatement" &&
			node.alternate !== undefined &&
			isNegation(node.test)
		) {
			return path.context.addFixableDiagnostic(
				{
					old: node,
					fixed: {
						...node,
						test: node.test.argument,
						consequent: node.alternate,
						alternate: node.consequent,
					},
				},
				descriptions.LINT.JS_NEGATION_ELSE,
			);
		}

		if (node.type === "ConditionalExpression" && isNegation(node.test)) {
			return path.context.addFixableDiagnostic(
				{
					old: node,
					fixed: {
						...node,
						test: node.test.argument,
						consequent: node.alternate,
						alternate: node.consequent,
					},
				},
				descriptions.LINT.JS_NEGATION_ELSE,
			);
		}

		return node;
	},
};
