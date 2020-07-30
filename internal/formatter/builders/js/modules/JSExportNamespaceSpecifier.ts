/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token, concat, space} from "@internal/formatter";

import {JSExportNamespaceSpecifier} from "@internal/ast";

export default function JSExportNamespaceSpecifier(
	builder: Builder,
	node: JSExportNamespaceSpecifier,
): Token {
	return concat(["*", space, "as", space, builder.tokenize(node.exported, node)]);
}
