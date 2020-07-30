/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token, concat} from "@internal/formatter";

import {JSComputedMemberProperty} from "@internal/ast";

export default function JSComputedMemberProperty(
	builder: Builder,
	node: JSComputedMemberProperty,
): Token {
	const tokens: Array<Token> = [];

	if (node.optional) {
		tokens.push("?.");
	}

	tokens.push("[", builder.tokenize(node.value, node), "]");

	return concat(tokens);
}
