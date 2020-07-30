/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSTypePredicate} from "@internal/ast";
import {Builder, Token, concat, space} from "@internal/formatter";

export default function TSTypePredicate(
	builder: Builder,
	node: TSTypePredicate,
): Token {
	const tokens: Array<Token> = [];

	if (node.asserts) {
		tokens.push("asserts", space);
	}

	tokens.push(builder.tokenize(node.parameterName, node));

	if (node.typeAnnotation) {
		tokens.push(space, "is", space, builder.tokenize(node.typeAnnotation, node));
	}

	return concat(tokens);
}
