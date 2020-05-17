/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from "../../Builder";
import {Token, concat} from "../../tokens";
import {ComputedMemberProperty} from "@romejs/js-ast";

export default function ComputedMemberProperty(
	builder: Builder,
	node: ComputedMemberProperty,
): Token {
	const tokens: Array<Token> = [];

	if (node.optional) {
		tokens.push("?.");
	}

	tokens.push("[", builder.tokenize(node.value, node), "]");

	return concat(tokens);
}
