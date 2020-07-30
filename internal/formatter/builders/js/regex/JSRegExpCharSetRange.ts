/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token, concat} from "@romefrontend/formatter";

import {JSRegExpCharSetRange} from "@romefrontend/ast";

export default function JSRegExpCharSetRange(
	builder: Builder,
	node: JSRegExpCharSetRange,
): Token {
	return concat([
		builder.tokenize(node.start, node),
		"-",
		builder.tokenize(node.end, node),
	]);
}
