/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSDeclareFunction} from "@romefrontend/ast";
import {Builder, Token, concat, space} from "@romefrontend/formatter";

export default function TSDeclareFunction(
	builder: Builder,
	node: TSDeclareFunction,
): Token {
	let tokens: Array<Token> = [];

	if (node.declare) {
		tokens.push("declare", space);
	}

	return concat([
		concat(tokens),
		"function",
		space,
		builder.tokenize(node.id, node),
		builder.tokenize(node.head, node),
		";",
	]);
}
