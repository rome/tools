/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSExportExternalDeclaration} from "@internal/ast";
import {Builder, Token, concat, group, space} from "@internal/formatter";

import {printModuleSpecifiers} from "./JSImportDeclaration";

export default function JSExportExternalDeclaration(
	builder: Builder,
	node: JSExportExternalDeclaration,
): Token {
	const tokens: Array<Token> = ["export", space];

	if (node.exportKind === "type") {
		if (!builder.options.typeAnnotations) {
			return "";
		}

		tokens.push("type", space);
	}

	tokens.push(
		printModuleSpecifiers(builder, node),
		space,
		"from",
		space,
		builder.tokenize(node.source, node),
		";",
	);

	return group(concat(tokens));
}
