/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from "../../Builder";
import {Token, concat, space} from "../../tokens";
import {ImportSpecifier} from "@romejs/js-ast";

export default function ImportSpecifier(
	builder: Builder,
	node: ImportSpecifier,
): Token {
	const tokens: Array<Token> = [];

	tokens.push(builder.tokenize(node.imported, node));

	if (node.local.name.name !== node.imported.name) {
		tokens.push(space, "as", space, builder.tokenize(node.local.name, node));
	}

	return concat(tokens);
}
