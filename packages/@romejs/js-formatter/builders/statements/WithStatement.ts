/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {WithStatement} from "@romejs/js-ast";
import Builder from "../../Builder";
import {Token, concat, space} from "../../tokens";
import {printClause} from "../utils";

export default function WithStatement(
	builder: Builder,
	node: WithStatement,
): Token {
	return concat([
		"with",
		space,
		"(",
		builder.tokenize(node.object, node),
		")",
		printClause(builder, node.body, node),
	]);
}
