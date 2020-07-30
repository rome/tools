/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSModuleBlock} from "@internal/ast";
import {Builder, Token, concat, hardline, indent} from "@internal/formatter";

export default function TSModuleBlock(
	builder: Builder,
	node: TSModuleBlock,
): Token {
	return concat([
		"{",
		indent(builder.tokenizeStatementList(node.body, node), true),
		hardline,
		"}",
	]);
}
