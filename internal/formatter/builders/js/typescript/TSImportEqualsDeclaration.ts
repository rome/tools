/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSImportEqualsDeclaration} from "@internal/ast";
import {Builder, Token, concat, group, space} from "@internal/formatter";

export default function TSImportEqualsDeclaration(
	builder: Builder,
	node: TSImportEqualsDeclaration,
): Token {
	const tokens: Array<Token> = [];
	if (node.isExport) {
		tokens.push("export");
		tokens.push(space);
	}

	tokens.push(
		"import",
		space,
		builder.tokenize(node.id, node),
		space,
		"=",
		space,
		builder.tokenize(node.moduleReference, node),
		";",
	);

	return group(concat(tokens));
}
