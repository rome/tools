/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSMethodSignature} from "@romejs/js-ast";
import {Builder} from "@romejs/js-formatter";
import {Token, concat, group, space} from "../../tokens";

export default function TSMethodSignature(
	builder: Builder,
	node: TSMethodSignature,
): Token {
	const tokens: Array<Token> = [
		builder.tokenize(node.key, node),
		builder.tokenize(node.meta, node),
	];

	if (node.returnType) {
		tokens.push(":");
		tokens.push(space);
		tokens.push(builder.tokenize(node.returnType, node));
	}

	tokens.push(";");

	return group(concat(tokens));
}
