/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token, concat} from "@internal/formatter";

import {JSUpdateExpression} from "@internal/ast";

export default function JSUpdateExpression(
	builder: Builder,
	node: JSUpdateExpression,
): Token {
	if (node.prefix === true) {
		return concat([node.operator, builder.tokenize(node.argument, node)]);
	} else {
		return concat([builder.tokenize(node.argument, node), node.operator]);
	}
}
