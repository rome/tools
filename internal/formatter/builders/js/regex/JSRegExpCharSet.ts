/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token, concat} from "@internal/formatter";

import {JSRegExpCharSet} from "@internal/ast";

export default function JSRegExpCharSet(
	builder: Builder,
	node: JSRegExpCharSet,
): Token {
	const tokens: Array<Token> = ["["];

	if (node.invert) {
		tokens.push("^");
	}

	return concat([
		concat(tokens),
		concat(node.body.map((item) => builder.tokenize(item, node))),
		"]",
	]);
}
