/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token, concat, group, space} from "@internal/formatter";
import {JSForOfStatement} from "@internal/ast";

import {printClause} from "../utils";

export default function JSForOfStatement(
	builder: Builder,
	node: JSForOfStatement,
): Token {
	return group(
		concat([
			"for",
			node.await ? concat([space, "await"]) : "",
			space,
			"(",
			builder.tokenize(node.left, node),
			space,
			"of",
			space,
			builder.tokenize(node.right, node),
			")",
			printClause(builder, node.body, node),
		]),
	);
}
