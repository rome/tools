/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSTypeParameter} from "@internal/ast";
import {Builder, Token, concat, space} from "@internal/formatter";

export default function TSTypeParameter(
	builder: Builder,
	node: TSTypeParameter,
): Token {
	const tokens: Array<Token> = [node.name];

	if (node.constraint) {
		tokens.push(
			space,
			"extends",
			space,
			builder.tokenize(node.constraint, node),
		);
	}

	if (node.default) {
		tokens.push(space, "=", space, builder.tokenize(node.default, node));
	}

	return concat(tokens);
}
