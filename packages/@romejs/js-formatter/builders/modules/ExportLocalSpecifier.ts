/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from "../../Builder";
import {Token, concat, space} from "../../tokens";
import {ExportExternalSpecifier, ExportLocalSpecifier} from "@romejs/js-ast";

export default function ExportLocalSpecifier(
	builder: Builder,
	node: ExportExternalSpecifier | ExportLocalSpecifier,
): Token {
	const tokens = [builder.tokenize(node.local, node)];

	if (node.local.name === node.exported.name) {
		return concat(tokens);
	} else {
		return concat([
			concat(tokens),
			space,
			"as",
			space,
			builder.tokenize(node.exported, node),
		]);
	}
}
