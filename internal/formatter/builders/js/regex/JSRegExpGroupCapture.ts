/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token, concat} from "@internal/formatter";

import {JSRegExpGroupCapture} from "@internal/ast";

export default function JSRegExpGroupCapture(
	builder: Builder,
	node: JSRegExpGroupCapture,
): Token {
	const tokens: Array<Token> = ["("];

	if (node.name !== undefined) {
		tokens.push("?<");
		tokens.push(node.name);
		tokens.push(">");
	}

	return concat([concat(tokens), builder.tokenize(node.expression, node), ")"]);
}
