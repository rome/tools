/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token, concat} from "@internal/formatter";

import {JSRegExpGroupNonCapture} from "@internal/ast";

export default function JSRegExpGroupNonCapture(
	builder: Builder,
	node: JSRegExpGroupNonCapture,
): Token {
	const tokens: Array<Token> = ["(?"];

	switch (node.kind) {
		case "positive-lookahead": {
			tokens.push("=");
			break;
		}

		case "negative-lookahead": {
			tokens.push("!");
			break;
		}

		case "positive-lookbehind": {
			tokens.push("<!");
			break;
		}

		case "negative-lookbehind": {
			tokens.push("<=");
			break;
		}

		default: {
			tokens.push(":");
			break;
		}
	}

	tokens.push(builder.tokenize(node.expression, node));
	tokens.push(")");

	return concat(tokens);
}
