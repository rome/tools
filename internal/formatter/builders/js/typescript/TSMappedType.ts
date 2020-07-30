/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSMappedType} from "@internal/ast";
import {
	Builder,
	Token,
	concat,
	group,
	indent,
	softline,
	space,
} from "@internal/formatter";

export default function TSMappedType(
	builder: Builder,
	node: TSMappedType,
): Token {
	const tokens: Array<Token> = [];

	if (node.readonly) {
		tokens.push(tokenIfPlusMinus(builder, node.readonly), "readonly", space);
	}

	const {typeParameter} = node;
	tokens.push(
		"[",
		typeParameter.name,
		space,
		"in",
		space,
		builder.tokenize(typeParameter.constraint, typeParameter),
		"]",
	);

	if (node.optional) {
		tokens.push(tokenIfPlusMinus(builder, node.optional), "?");
	}

	if (node.typeAnnotation) {
		tokens.push(":", space, builder.tokenize(node.typeAnnotation, node));
	}

	return group(
		concat(["{", indent(concat([softline, concat(tokens)])), softline, "}"]),
	);
}

function tokenIfPlusMinus(builder: Builder, token: string | true): Token {
	switch (token) {
		case "+":
		case "-":
			return token;

		default:
			return "";
	}
}
