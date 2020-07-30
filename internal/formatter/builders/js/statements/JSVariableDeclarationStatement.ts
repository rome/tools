/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token, concat, space} from "@internal/formatter";
import {JSVariableDeclarationStatement} from "@internal/ast";

export default function JSVariableDeclarationStatement(
	builder: Builder,
	node: JSVariableDeclarationStatement,
): Token {
	if (node.declare === true && !builder.options.typeAnnotations) {
		return "";
	}

	const tokens: Array<Token> = [];

	if (node.declare) {
		tokens.push("declare", space);
	}

	return concat([concat(tokens), builder.tokenize(node.declaration, node), ";"]);
}
