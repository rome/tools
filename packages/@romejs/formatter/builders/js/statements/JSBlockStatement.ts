/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token, concat, hardline, indent} from "@romejs/formatter";

import {AnyNode, JSBlockStatement} from "@romejs/ast";

export default function JSBlockStatement(
	builder: Builder,
	node: JSBlockStatement,
	parent: AnyNode,
): Token {
	const hasComments =
		node.innerComments !== undefined && node.innerComments.length > 0;
	const hasContents = node.body !== undefined && node.body.length > 0;
	const hasDirectives =
		node.directives !== undefined && node.directives.length > 0;

	if (
		!hasComments &&
		!hasContents &&
		!hasDirectives &&
		(parent.type === "JSArrowFunctionExpression" ||
		parent.type === "JSClassMethod" ||
		parent.type === "JSClassPrivateMethod" ||
		parent.type === "JSDoWhileStatement" ||
		parent.type === "JSForInStatement" ||
		parent.type === "JSForOfStatement" ||
		parent.type === "JSForStatement" ||
		parent.type === "JSFunctionDeclaration" ||
		parent.type === "JSFunctionExpression" ||
		parent.type === "JSObjectMethod" ||
		parent.type === "JSSwitchStatement" ||
		parent.type === "JSWhileStatement")
	) {
		return "{}";
	}

	const tokens: Array<Token> = ["{"];

	if (hasDirectives) {
		for (const directive of node.directives!) {
			tokens.push(indent(concat([hardline, builder.tokenize(directive, node)])));
		}
	}

	if (hasContents) {
		tokens.push(
			indent(concat([hardline, builder.tokenizeStatementList(node.body, node)])),
		);
	}

	if (hasComments) {
		tokens.push(builder.tokenizeInnerComments(node, true));
	}

	tokens.push(hardline, "}");

	return concat(tokens);
}
