/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode, BinaryExpression, LogicalExpression} from "@romejs/js-ast";
import {getPrecedence, isBinary} from "@romejs/js-ast-utils";
import Builder from "../../Builder";
import {Token, concat, group, lineOrSpace, space} from "../../tokens";

export default function BinaryExpression(
	builder: Builder,
	node: BinaryExpression | LogicalExpression,
	parent: AnyNode,
): Token {
	const shouldNotGroup =
		(parent.type === "IfStatement" && parent.test === node) ||
		(parent.type === "DoWhileStatement" && parent.test === node) ||
		(parent.type === "WhileStatement" && parent.test === node) ||
		(parent.type === "SwitchStatement" && parent.discriminant === node);

	const parts = printBinaryExpression(builder, node, parent, shouldNotGroup);

	if (shouldNotGroup) {
		return concat(parts);
	}

	return group(concat(parts));
}

function printBinaryExpression(
	builder: Builder,
	node: BinaryExpression | LogicalExpression,
	parent: AnyNode,
	shouldNotGroup: boolean,
): Array<Token> {
	const parts: Array<Token> = [];

	if (
		isBinary(node.left) &&
		getPrecedence(node.operator) === getPrecedence(node.left.operator)
	) {
		parts.push(...printBinaryExpression(builder, node.left, node, shouldNotGroup));
	} else {
		parts.push(builder.tokenize(node.left, node));
	}

	// Inline object and array expressions:
	//   obj && {
	//   arr ?? [
	const shouldInline =
		node.type === "LogicalExpression" &&
		(node.right.type === "ArrayExpression" ||
		node.right.type === "ObjectExpression");

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
