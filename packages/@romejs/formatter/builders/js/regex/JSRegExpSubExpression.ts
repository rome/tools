/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token, concat} from "@romejs/formatter";

import {JSRegExpSubExpression} from "@romejs/ast";

export default function JSRegExpSubExpression(
	builder: Builder,
	node: JSRegExpSubExpression,
): Token {
	return concat(node.body.map((item) => builder.tokenize(item, node)));
}
