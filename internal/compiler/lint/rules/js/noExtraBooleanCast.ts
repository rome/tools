/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {createLintVisitor, signals} from "@internal/compiler";
import {
	AnyNode,
	JSCallExpression,
	JSNewExpression,
	JSUnaryExpression,
} from "@internal/ast";
import {descriptions} from "@internal/diagnostics";

function isBooleanConstructorCall(node: AnyNode): node is JSNewExpression {
	return (
		node.type === "JSNewExpression" &&
		node.callee.type === "JSReferenceIdentifier" &&
		node.callee.name === "Boolean"
	);
}

function isBooleanCall(node: AnyNode): node is JSCallExpression {
	return (
		node.type === "JSCallExpression" &&
		node.callee.type === "JSReferenceIdentifier" &&
		node.callee.name === "Boolean"
	);
}

function isInBooleanContext(node: AnyNode, parent: AnyNode): boolean {
	return (
		(parent.type === "JSIfStatement" ||
		parent.type === "JSDoWhileStatement" ||
		parent.type === "JSWhileStatement" ||
		parent.type === "JSForStatement" ||
		parent.type === "JSConditionalExpression") &&
		parent.test === node
	);
}

function isNegation(node: AnyNode): node is JSUnaryExpression {
	return node.type === "JSUnaryExpression" && node.operator === "!";
}

export default createLintVisitor({
	name: "js/noExtraBooleanCast",
	enter(path) {
		const {node, parent} = path;

		if (
			isInBooleanContext(node, parent) ||
			isBooleanConstructorCall(parent) ||
			isNegation(parent) ||
			isBooleanCall(parent)
		) {
			if (isNegation(node) && isNegation(node.argument)) {
				return path.addFixableDiagnostic(
					{
						fixed: signals.replace(node.argument.argument),
					},
					descriptions.LINT.JS_NO_EXTRA_BOOLEAN_CAST,
				);
			}

			if (isBooleanCall(node) && node.arguments[0]) {
				return path.addFixableDiagnostic(
					{
						fixed: signals.replace(node.arguments[0]),
					},
					descriptions.LINT.JS_NO_EXTRA_BOOLEAN_CAST,
				);
			}
		}

		return signals.retain;
	},
});
