/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token, concat} from "@romejs/formatter";

import {JSXSpreadAttribute} from "@romejs/ast";

export default function JSXSpreadAttribute(
	builder: Builder,
	node: JSXSpreadAttribute,
): Token {
	return concat(["{", "...", builder.tokenize(node.argument, node), "}"]);
}
