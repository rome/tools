/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token, concat, group, space} from "@internal/formatter";
import {JSForInStatement} from "@internal/ast";

import {printClause} from "../utils";

export default function JSForInStatement(
	builder: Builder,
	node: JSForInStatement,
): Token {
	return group(
		concat([
			"for",
			space,
			"(",
			builder.tokenize(node.left, node),
			space,
			"in",
			space,
			builder.tokenize(node.right, node),
			")",
			printClause(builder, node.body, node),
		]),
	);
}
