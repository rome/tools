/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token, concat} from "@internal/formatter";

import {JSMemberExpression} from "@internal/ast";

export default function JSMemberExpression(
	builder: Builder,
	node: JSMemberExpression,
): Token {
	return concat([
		builder.tokenize(node.object, node),
		builder.tokenize(node.property, node),
	]);
}
