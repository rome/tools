/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from "../../Builder";
import {VariableDeclaration} from "@romejs/js-ast";
import {Token, concat, group, indent, lineOrSpace, space} from "../../tokens";

export default function VariableDeclaration(
	builder: Builder,
	node: VariableDeclaration,
): Token {
	const declarations = node.declarations.map((declaration) =>
		builder.tokenize(declaration, node)
	);

	return group(
		concat([
			node.kind,
			space,
			declarations.shift()!,
			indent(
				concat(
					declarations.map((declaration) => concat([",", lineOrSpace, declaration])),
				),
			),
		]),
	);
}
