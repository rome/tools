/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token, concat, space} from "@romejs/formatter";

import {JSDoExpression} from "@romejs/ast";

export default function JSDoExpression(
	builder: Builder,
	node: JSDoExpression,
): Token {
	return concat(["do", space, builder.tokenize(node.body, node)]);
}
