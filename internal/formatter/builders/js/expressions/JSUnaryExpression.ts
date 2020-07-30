/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token, concat, space} from "@internal/formatter";

import {JSUnaryExpression} from "@internal/ast";

export default function JSUnaryExpression(
	builder: Builder,
	node: JSUnaryExpression,
): Token {
	if (
		node.operator === "void" ||
		node.operator === "delete" ||
		node.operator === "typeof"
	) {
		return concat([node.operator, space, builder.tokenize(node.argument, node)]);
	} else {
		return concat([node.operator, builder.tokenize(node.argument, node)]);
	}
}
