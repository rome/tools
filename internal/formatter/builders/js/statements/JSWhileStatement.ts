/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSWhileStatement} from "@internal/ast";
import {
	Builder,
	Token,
	concat,
	group,
	indent,
	softline,
	space,
} from "@internal/formatter";

import {printClause} from "../utils";

export default function JSWhileStatement(
	builder: Builder,
	node: JSWhileStatement,
): Token {
	return group(
		concat([
			"while",
			space,
			"(",
			group(
				concat([
					indent(concat([softline, builder.tokenize(node.test, node)])),
					softline,
				]),
			),
			")",
			printClause(builder, node.body, node),
		]),
	);
}
