/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {WhileStatement} from "@romejs/js-ast";
import Builder from "../../Builder";
import {Token, concat, group, indent, softline, space} from "../../tokens";
import {printClause} from "../utils";

export default function WhileStatement(
	builder: Builder,
	node: WhileStatement,
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
