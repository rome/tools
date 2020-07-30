/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSImportType} from "@internal/ast";
import {Builder, Token, concat} from "@internal/formatter";

export default function TSImportType(
	builder: Builder,
	node: TSImportType,
): Token {
	const tokens: Array<Token> = [
		"import(",
		builder.tokenize(node.argument, node),
		")",
	];

	if (node.qualifier) {
		tokens.push(".", builder.tokenize(node.qualifier, node));
	}

	if (node.typeParameters) {
		tokens.push(builder.tokenize(node.typeParameters, node));
	}

	return concat(tokens);
}
