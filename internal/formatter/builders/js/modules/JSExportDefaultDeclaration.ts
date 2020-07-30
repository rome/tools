/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSExportDefaultDeclaration} from "@internal/ast";
import {Builder, Token, concat, space} from "@internal/formatter";

import {printExportDeclaration} from "./JSExportLocalDeclaration";

export default function JSExportDefaultDeclaration(
	builder: Builder,
	node: JSExportDefaultDeclaration,
): Token {
	return concat([
		"export",
		space,
		"default",
		space,
		printExportDeclaration(builder, node),
	]);
}
