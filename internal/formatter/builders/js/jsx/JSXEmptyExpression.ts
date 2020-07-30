/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token} from "@internal/formatter";

import {JSXEmptyExpression} from "@internal/ast";

export default function JSXEmptyExpression(
	builder: Builder,
	node: JSXEmptyExpression,
): Token {
	return builder.tokenizeInnerComments(node, false);
}
