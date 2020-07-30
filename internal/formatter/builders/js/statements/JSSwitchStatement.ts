/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	Builder,
	Token,
	concat,
	group,
	hardline,
	indent,
	softline,
	space,
} from "@internal/formatter";

import {JSSwitchStatement} from "@internal/ast";

export default function JSSwitchStatement(
	builder: Builder,
	node: JSSwitchStatement,
): Token {
	return concat([
		group(
			concat([
				"switch",
				space,
				"(",
				group(
					concat([
						indent(
							concat([softline, builder.tokenize(node.discriminant, node)]),
						),
						softline,
					]),
				),
				")",
			]),
		),
		space,
		"{",
		node.cases.length > 0
			? indent(
					concat([hardline, builder.tokenizeStatementList(node.cases, node)]),
				)
			: "",
		hardline,
		"}",
	]);
}
