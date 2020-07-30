/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode, JSBinaryExpression, JSLogicalExpression} from "@internal/ast";
import {getPrecedence, isBinary} from "@internal/js-ast-utils";
import {
	Builder,
	Token,
	concat,
	group,
	lineOrSpace,
	space,
} from "@internal/formatter";

export default function JSBinaryExpression(
	builder: Builder,
	node: JSBinaryExpression | JSLogicalExpression,
	parent: AnyNode,
): Token {
	const shouldNotGroup =
		(parent.type === "JSIfStatement" && parent.test === node) ||
		(parent.type === "JSDoWhileStatement" && parent.test === node) ||
		(parent.type === "JSWhileStatement" && parent.test === node) ||
		(parent.type === "JSSwitchStatement" && parent.discriminant === node);

	const parts = printBinaryExpression(builder, node, parent, shouldNotGroup);

	if (shouldNotGroup) {
		return concat(parts);
	}

	return group(concat(parts));
}

function printBinaryExpression(
	builder: Builder,
	node: JSBinaryExpression | JSLogicalExpression,
	parent: AnyNode,
	shouldNotGroup: boolean,
): Array<Token> {
	const parts: Array<Token> = [];

	if (
		isBinary(node.left) &&
		getPrecedence(node.operator) === getPrecedence(node.left.operator)
	) {
		parts.push(
			...printBinaryExpression(builder, node.left, node, shouldNotGroup),
		);
	} else {
		parts.push(builder.tokenize(node.left, node));
	}

	// Inline object and array expressions:
	//   obj && {
	//   arr ?? [
	const shouldInline =
		node.type === "JSLogicalExpression" &&
		(node.right.type === "JSArrayExpression" ||
		node.right.type === "JSObjectExpression");

	const right = concat([
		node.operator,
		shouldInline ? space : lineOrSpace,
		builder.tokenize(node.right, node),
	]);

	const shouldGroup =
		!shouldNotGroup &&
		node.type !== parent.type &&
		node.type !== node.left.type &&
		node.type !== node.right.type;

	parts.push(concat([space, shouldGroup ? group(right) : right]));

	return parts;
}
