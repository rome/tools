/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token, concat, space} from "@internal/formatter";

import {JSCatchClause} from "@internal/ast";

export default function JSCatchClause(
	builder: Builder,
	node: JSCatchClause,
): Token {
	if (node.param) {
		return concat([
			"catch",
			space,
			"(",
			builder.tokenize(node.param, node),
			") ",
			builder.tokenize(node.body, node),
		]);
	} else {
		return concat(["catch", space, builder.tokenize(node.body, node)]);
	}
}
