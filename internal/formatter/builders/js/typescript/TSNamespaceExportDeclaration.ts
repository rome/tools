/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSNamespaceExportDeclaration} from "@internal/ast";
import {Builder, Token, concat, space} from "@internal/formatter";

export default function TSNamespaceExportDeclaration(
	builder: Builder,
	node: TSNamespaceExportDeclaration,
): Token {
	return concat([
		"export",
		space,
		"as",
		space,
		"namespace",
		space,
		builder.tokenize(node.id, node),
		";",
	]);
}
