/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSModuleBlock, TSModuleDeclaration} from "@internal/ast";
import {Builder, Token, concat, space} from "@internal/formatter";

export default function TSModuleDeclaration(
	builder: Builder,
	node: TSModuleDeclaration,
): Token {
	const tokens: Array<Token> = [];

	if (node.declare) {
		tokens.push("declare");
		tokens.push(space);
	}

	if (!node.global) {
		tokens.push(node.id.type === "JSBindingIdentifier" ? "namespace" : "module");
		tokens.push(space);
	}

	tokens.push(builder.tokenize(node.id, node));

	if (!node.body) {
		tokens.push(";");
		return concat(tokens);
	}

	let body: undefined | TSModuleBlock | TSModuleDeclaration = node.body;
	while (body !== undefined && body.type === "TSModuleDeclaration") {
		tokens.push(".", builder.tokenize(body.id, body));
		body = body.body;
	}

	return concat([concat(tokens), space, builder.tokenize(body, node)]);
}
