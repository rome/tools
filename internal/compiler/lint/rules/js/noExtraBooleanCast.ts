/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path, createVisitor, signals} from "@internal/compiler";
import {
	AnyNode,
	JSConditionalExpression,
	JSDoWhileStatement,
	JSForStatement,
	JSIfStatement,
	JSWhileStatement,
} from "@internal/ast";
import {descriptions} from "@internal/diagnostics";

function isBooleanConstructorCall(node: AnyNode) {
	return (
		node.type === "JSNewExpression" &&
		node.callee.type === "JSReferenceIdentifier" &&
		node.callee.name === "Boolean"
	);
}

function isConditionalStatement(node: AnyNode): node is JSConditionalExpression {
	return node.type === "JSConditionalExpression";
}

function isInBooleanContext(
	node: AnyNode,
): node is
	| JSIfStatement
	| JSDoWhileStatement
	| JSWhileStatement
	| JSForStatement {
	return (
		node.type === "JSIfStatement" ||
		node.type === "JSDoWhileStatement" ||
		node.type === "JSWhileStatement" ||
		node.type === "JSForStatement"
	);
}

function getNode(path: Path): undefined | AnyNode {
	let {node} = path;

	if (isBooleanConstructorCall(node)) {
		if (node.type === "JSNewExpression" && node.arguments.length > 0) {
			return node.arguments[0];
		}
	}

	if (isInBooleanContext(node) || isConditionalStatement(node)) {
		return node.test;
	}

	return undefined;
}

export default createVisitor({
	name: "js/noExtraBooleanCast",
	enter(path) {
		const {context} = path;

		let node = getNode(path);

		if (node !== undefined) {
			if (
				(node.type === "JSUnaryExpression" &&
				node.operator === "!" &&
				node.argument.type === "JSUnaryExpression" &&
				node.argument.operator === "!") ||
				(node.type === "JSCallExpression" &&
				node.callee.type === "JSReferenceIdentifier" &&
				node.callee.name === "Boolean")
			) {
				context.addNodeDiagnostic(
					node,
					descriptions.LINT.JS_NO_EXTRA_BOOLEAN_CAST,
				);
			}
		}

		return signals.retain;
	},
});
