/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {createVisitor, signals} from "@internal/compiler";
import {AnyJSExpression, JSUnaryExpression} from "@internal/ast";
import {descriptions} from "@internal/diagnostics";

function isNegation(node: AnyJSExpression): node is JSUnaryExpression {
	return (
		node.type === "JSUnaryExpression" &&
		node.prefix === true &&
		node.operator === "!"
	);
}

export default createVisitor({
	name: "js/noNegationElse",
	enter(path) {
		const {node} = path;

		if (
			node.type === "JSIfStatement" &&
			node.alternate !== undefined &&
			isNegation(node.test)
		) {
			return path.addFixableDiagnostic(
				{
					fixed: signals.replace({
						...node,
						test: node.test.argument,
						consequent: node.alternate,
						alternate: node.consequent,
					}),
				},
				descriptions.LINT.JS_NO_NEGATION_ELSE,
			);
		}

		if (node.type === "JSConditionalExpression" && isNegation(node.test)) {
			return path.addFixableDiagnostic(
				{
					fixed: signals.replace({
						...node,
						test: node.test.argument,
						consequent: node.alternate,
						alternate: node.consequent,
					}),
				},
				descriptions.LINT.JS_NO_NEGATION_ELSE,
			);
		}

		return signals.retain;
	},
});
