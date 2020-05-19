/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token, concat} from "@romejs/formatter";

import {JSExpressionStatement} from "@romejs/ast";

export default function JSExpressionStatement(
	builder: Builder,
	node: JSExpressionStatement,
): Token {
	return concat([builder.tokenize(node.expression, node), ";"]);
}
