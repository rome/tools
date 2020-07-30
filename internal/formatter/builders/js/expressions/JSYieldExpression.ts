/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token, concat, space} from "@internal/formatter";

import {JSYieldExpression} from "@internal/ast";

export default function JSYieldExpression(
	builder: Builder,
	node: JSYieldExpression,
): Token {
	const tokens: Array<Token> = ["yield"];

	if (node.delegate === true) {
		tokens.push("*");
	}

	if (node.argument) {
		tokens.push(space, builder.tokenize(node.argument, node));
	}

	return concat(tokens);
}
