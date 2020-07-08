/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token, concat, space} from "@romefrontend/formatter";

import {JSImportNamespaceSpecifier} from "@romefrontend/ast";

export default function JSImportNamespaceSpecifier(
	builder: Builder,
	node: JSImportNamespaceSpecifier,
): Token {
	return concat([
		"*",
		space,
		"as",
		space,
		builder.tokenize(node.local.name, node),
	]);
}
