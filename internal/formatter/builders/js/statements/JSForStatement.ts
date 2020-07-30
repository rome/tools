/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSForStatement} from "@internal/ast";
import {
	Builder,
	Token,
	concat,
	group,
	indent,
	lineOrSpace,
	softline,
	space,
} from "@internal/formatter";

import {printClause} from "../utils";

export default function JSForStatement(
	builder: Builder,
	node: JSForStatement,
): Token {
	const body = printClause(builder, node.body, node);

	if (!node.init && !node.test && !node.update) {
		return group(concat(["for", space, "(;;)", body]));
	}

	return group(
		concat([
			"for",
			space,
			"(",
			group(
				concat([
					indent(
						concat([
							softline,
							builder.tokenize(node.init, node),
							";",
							lineOrSpace,
							builder.tokenize(node.test, node),
							";",
							lineOrSpace,
							builder.tokenize(node.update, node),
						]),
					),
					softline,
				]),
			),
			")",
			body,
		]),
	);
}
