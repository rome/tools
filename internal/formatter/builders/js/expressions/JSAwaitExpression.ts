/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token, concat, space} from "@internal/formatter";

import {JSAwaitExpression} from "@internal/ast";

export default function JSAwaitExpression(
	builder: Builder,
	node: JSAwaitExpression,
): Token {
	return concat(["await", space, builder.tokenize(node.argument, node)]);
}
