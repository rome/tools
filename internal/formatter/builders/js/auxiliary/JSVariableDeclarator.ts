/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token, concat, space} from "@internal/formatter";
import {JSVariableDeclarator} from "@internal/ast";

import {printAssignment} from "../utils";

export default function JSVariableDeclarator(
	builder: Builder,
	node: JSVariableDeclarator,
): Token {
	if (node.init) {
		return printAssignment(
			builder,
			node,
			node.id,
			concat([space, "="]),
			node.init,
		);
	} else {
		return builder.tokenize(node.id, node);
	}
}
