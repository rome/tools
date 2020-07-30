/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSPropertySignature} from "@internal/ast";
import {Builder, Token, concat, space} from "@internal/formatter";

export default function TSPropertySignature(
	builder: Builder,
	node: TSPropertySignature,
): Token {
	const tokens: Array<Token> = [];

	if (node.readonly) {
		tokens.push("readonly", space);
	}

	tokens.push(builder.tokenize(node.key, node));

	if (node.optional) {
		tokens.push("?");
	}

	if (node.typeAnnotation) {
		tokens.push(":", space, builder.tokenize(node.typeAnnotation, node));
	}

	tokens.push(";");

	return concat(tokens);
}
