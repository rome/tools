/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token, concat} from "@internal/formatter";

import {JSXExpressionContainer} from "@internal/ast";

export default function JSXExpressionContainer(
	builder: Builder,
	node: JSXExpressionContainer,
): Token {
	return concat(["{", builder.tokenize(node.expression, node), "}"]);
}
