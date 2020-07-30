/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSDoWhileStatement} from "@internal/ast";
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

import {printClause} from "../utils";

export default function JSDoWhileStatement(
	builder: Builder,
	node: JSDoWhileStatement,
): Token {
	return concat([
		group(concat(["do", printClause(builder, node.body, node)])),
		node.body.type === "JSBlockStatement" ? space : hardline,
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
		";",
	]);
}
