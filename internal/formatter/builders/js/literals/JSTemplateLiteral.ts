/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token, concat} from "@internal/formatter";
import {JSTemplateLiteral} from "@internal/ast";

export default function JSTemplateLiteral(
	builder: Builder,
	node: JSTemplateLiteral,
): Token {
	const tokens: Array<Token> = [];
	const quasis = node.quasis;

	for (let i = 0; i < quasis.length; i++) {
		tokens.push(builder.tokenize(quasis[i], node));

		if (i + 1 < quasis.length) {
			tokens.push(builder.tokenize(node.expressions[i], node));
		}
	}

	return concat(tokens);
}
