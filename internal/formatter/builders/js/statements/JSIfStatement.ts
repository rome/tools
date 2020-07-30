/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	Builder,
	Token,
	concat,
	group,
	hardline,
	indent,
	softline,
	space,
} from "@internal/formatter";

import {AnyNode, JSIfStatement} from "@internal/ast";
import {isStatement} from "@internal/js-ast-utils";

export default function JSIfStatement(
	builder: Builder,
	node: JSIfStatement,
): Token {
	const tokens: Array<Token> = [
		group(
			concat([
				"if",
				space,
				"(",
				group(
					concat([
						indent(concat([softline, builder.tokenize(node.test, node)])),
						softline,
					]),
				),
				")",
			]),
		),
		space,
	];

	let needsBlock = false;
	if (node.alternate) {
		needsBlock = getLastStatement(node.consequent).type === "JSIfStatement";
	}

	if (needsBlock) {
		tokens.push(
			"{",
			indent(builder.tokenize(node.consequent, node), true),
			hardline,
			"}",
		);
	} else {
		tokens.push(builder.tokenize(node.consequent, node));
	}

	if (node.alternate) {
		tokens.push(space, "else", space, builder.tokenize(node.alternate, node));
	}

	return concat(tokens);
}

// Recursively get the last statement.
function getLastStatement(statement: AnyNode): AnyNode {
	if (
		(statement.type === "JSWithStatement" ||
		statement.type === "JSWhileStatement" ||
		statement.type === "JSDoWhileStatement" ||
		statement.type === "JSForOfStatement" ||
		statement.type === "JSForInStatement" ||
		statement.type === "JSForStatement") &&
		isStatement(statement.body)
	) {
		return getLastStatement(statement.body);
	} else {
		return statement;
	}
}
