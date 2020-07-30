/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token, concat} from "@internal/formatter";

import {JSRegExpAlternation} from "@internal/ast";

export default function JSRegExpAlternation(
	builder: Builder,
	node: JSRegExpAlternation,
): Token {
	return concat([
		builder.tokenize(node.left, node),
		"|",
		builder.tokenize(node.right, node),
	]);
}
