/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSExportAllDeclaration} from "@internal/ast";
import {Builder, Token, concat, space} from "@internal/formatter";

export default function JSExportAllDeclaration(
	builder: Builder,
	node: JSExportAllDeclaration,
): Token {
	const tokens: Array<Token> = ["export", space];

	if (node.exportKind === "type") {
		if (!builder.options.typeAnnotations) {
			return "";
		}

		tokens.push("type", space);
	}

	tokens.push(
		"*",
		space,
		"from",
		space,
		builder.tokenize(node.source, node),
		";",
	);

	return concat(tokens);
}
