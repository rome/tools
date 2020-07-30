/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSExportAssignment} from "@internal/ast";
import {Builder, Token, concat, space} from "@internal/formatter";

export default function TSExportAssignment(
	builder: Builder,
	node: TSExportAssignment,
): Token {
	return concat([
		"export",
		space,
		"=",
		space,
		builder.tokenize(node.expression, node),
		";",
	]);
}
