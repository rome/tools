/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token, concat, space} from "@internal/formatter";

import {JSFunctionDeclaration, JSFunctionExpression} from "@internal/ast";

export default function JSFunctionExpression(
	builder: Builder,
	node: JSFunctionDeclaration | JSFunctionExpression,
): Token {
	const tokens: Array<Token> = [];

	if (node.head.async === true) {
		tokens.push("async");
		tokens.push(space);
	}

	tokens.push("function");

	if (node.head.generator === true) {
		tokens.push("*");
	}

	if (node.id) {
		tokens.push(space, builder.tokenize(node.id, node));
	}

	tokens.push(
		builder.tokenize(node.head, node),
		space,
		builder.tokenize(node.body, node),
	);

	return concat(tokens);
}
