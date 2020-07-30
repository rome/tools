/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSEnumDeclaration} from "@internal/ast";
import {Builder, Token, concat, space} from "@internal/formatter";

import {printTSBraced} from "../utils";

export default function TSEnumDeclaration(
	builder: Builder,
	node: TSEnumDeclaration,
): Token {
	const tokens: Array<Token> = [];

	if (node.declare) {
		tokens.push("declare", space);
	}

	if (node.const) {
		tokens.push("const", space);
	}

	tokens.push(
		"enum",
		space,
		builder.tokenize(node.id, node),
		space,
		printTSBraced(builder, node, node.members),
	);

	return concat(tokens);
}
