/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token, concat, space} from "@internal/formatter";

import {JSImportSpecifier} from "@internal/ast";

export default function JSImportSpecifier(
	builder: Builder,
	node: JSImportSpecifier,
): Token {
	const tokens: Array<Token> = [];

	tokens.push(builder.tokenize(node.imported, node));

	if (node.local.name.name !== node.imported.name) {
		tokens.push(space, "as", space, builder.tokenize(node.local.name, node));
	}

	return concat(tokens);
}
