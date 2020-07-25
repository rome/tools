/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from "@romefrontend/compiler";
import {AnyJSExpression, JSUnaryExpression} from "@romefrontend/ast";
import {descriptions} from "@romefrontend/diagnostics";

function isNegation(node: AnyJSExpression): node is JSUnaryExpression {
	return (
		node.type === "JSUnaryExpression" &&
		node.prefix === true &&
		node.operator === "!"
	);
}

export default {
	name: "js/negationElse",
	enter(path: Path) {
		const {node} = path;

		if (
			node.type === "JSIfStatement" &&
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

		if (node.type === "JSConditionalExpression" && isNegation(node.test)) {
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
