/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token, concat} from "@internal/formatter";

import {JSTaggedTemplateExpression} from "@internal/ast";

export default function JSTaggedTemplateExpression(
	builder: Builder,
	node: JSTaggedTemplateExpression,
): Token {
	const tokens: Token[] = [builder.tokenize(node.tag, node)];
	if (node.typeArguments) {
		tokens.push(builder.tokenize(node.typeArguments, node));
	}
	tokens.push(builder.tokenize(node.quasi, node));
	return concat(tokens);
}
