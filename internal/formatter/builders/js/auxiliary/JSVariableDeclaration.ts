/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	Builder,
	Token,
	concat,
	group,
	indent,
	lineOrSpace,
	space,
} from "@internal/formatter";
import {JSVariableDeclaration} from "@internal/ast";

export default function JSVariableDeclaration(
	builder: Builder,
	node: JSVariableDeclaration,
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
					declarations.map((declaration) =>
						concat([",", lineOrSpace, declaration])
					),
				),
			),
		]),
	);
}
