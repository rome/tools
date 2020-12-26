/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSTupleType} from "@internal/ast";
import {
	Builder,
	Token,
	concat,
	group,
	hardline,
	ifBreak,
	indent,
	join,
	lineOrSpace,
	softline,
} from "@internal/formatter";

import {hasInnerComments} from "../../comments";

export default function TSTupleType(builder: Builder, node: TSTupleType): Token {
	if (node.elementTypes.length === 0) {
		if (hasInnerComments(node)) {
			return concat([
				"[",
				builder.tokenizeInnerComments(node, true),
				hardline,
				"]",
			]);
		} else {
			return "[]";
		}
	}

	const parts: Token[] = [];

	for (const elementType of node.elementTypes) {
		parts.push(builder.tokenize(elementType, node));
	}

	const tokens: Token[] = [
		"[",
		indent(concat([softline, join(concat([",", lineOrSpace]), parts)])),
	];

	if (
		node.elementTypes[node.elementTypes.length - 1].typeAnnotation.type ===
		"TSRestType"
	) {
		tokens.push(ifBreak(","));
	}

	tokens.push(softline, "]");

	return group(concat(tokens));
}
