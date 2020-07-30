/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSWithStatement} from "@internal/ast";
import {Builder, Token, concat, space} from "@internal/formatter";

import {printClause} from "../utils";

export default function JSWithStatement(
	builder: Builder,
	node: JSWithStatement,
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
