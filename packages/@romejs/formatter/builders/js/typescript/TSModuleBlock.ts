/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSModuleBlock} from "@romejs/ast";
import {Builder, Token, concat, hardline, indent} from "@romejs/formatter";

export default function TSModuleBlock(
	builder: Builder,
	node: TSModuleBlock,
): Token {
	return concat([
		"{",
		indent(concat([hardline, builder.tokenizeStatementList(node.body, node)])),
		hardline,
		"}",
	]);
}
