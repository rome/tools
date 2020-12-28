/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSParenthesizedType} from "@internal/ast";
import {Builder, Token, concat} from "@internal/formatter";

export default function TSParenthesizedType(
	builder: Builder,
	node: TSParenthesizedType,
): Token {
	const tokens = builder.tokenize(node.typeAnnotation, node);
	if (node.typeAnnotation.type === "TSParenthesizedType") {
		return tokens;
	}
	return concat(["(", tokens, ")"]);
}
