/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from "../../Builder";
import {Token, concat, space} from "../../tokens";
import {TryStatement} from "@romejs/js-ast";

export default function TryStatement(
	builder: Builder,
	node: TryStatement,
): Token {
	const tokens: Array<Token> = ["try", space, builder.tokenize(node.block, node)];

	if (node.handler) {
		tokens.push(space, builder.tokenize(node.handler, node));
	}

	if (node.finalizer) {
		tokens.push(space, "finally", space, builder.tokenize(node.finalizer, node));
	}

	return concat(tokens);
}
