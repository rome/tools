/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token, concat} from "@internal/formatter";

import {JSXMemberExpression} from "@internal/ast";

export default function JSXMemberExpression(
	builder: Builder,
	node: JSXMemberExpression,
): Token {
	return concat([
		builder.tokenize(node.object, node),
		".",
		builder.tokenize(node.property, node),
	]);
}
