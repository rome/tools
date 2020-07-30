/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSCallSignatureDeclaration} from "@internal/ast";
import {Builder, Token, concat, space} from "@internal/formatter";

export default function TSCallSignatureDeclaration(
	builder: Builder,
	node: TSCallSignatureDeclaration,
): Token {
	const tokens: Array<Token> = [builder.tokenize(node.meta, node)];

	if (node.typeAnnotation) {
		tokens.push(":", space, builder.tokenize(node.typeAnnotation, node));
	}

	tokens.push(";");

	return concat(tokens);
}
