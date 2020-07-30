/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token, concat, space} from "@internal/formatter";

import {JSLabeledStatement} from "@internal/ast";

export default function JSLabeledStatement(
	builder: Builder,
	node: JSLabeledStatement,
): Token {
	return concat([
		builder.tokenize(node.label, node),
		":",
		node.body.type === "JSEmptyStatement"
			? ";"
			: concat([space, builder.tokenize(node.body, node)]),
	]);
}
