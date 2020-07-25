/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSNonNullExpression} from "@romefrontend/ast";
import {Builder, Token, concat} from "@romefrontend/formatter";

export default function TSNonNullExpression(
	builder: Builder,
	node: TSNonNullExpression,
): Token {
	const expr = builder.tokenize(node.expression, node);

	if (builder.options.typeAnnotations) {
		return concat([expr, "!"]);
	} else {
		return expr;
	}
}
