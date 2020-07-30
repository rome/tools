/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token, concat, hardline} from "@internal/formatter";
import {JSRoot} from "@internal/ast";

export default function JSRoot(builder: Builder, node: JSRoot): Token {
	const tokens: Array<Token> = [
		builder.tokenizeStatementList(node.directives, node),
	];

	if (node.directives && node.directives.length) {
		tokens.push(hardline);
	}

	if (node.interpreter && builder.options.allowInterpreterDirective) {
		tokens.push(builder.tokenize(node.interpreter, node));
	}

	tokens.push(
		builder.tokenizeInnerComments(node, false),
		builder.tokenizeStatementList(node.body, node),
	);

	tokens.push(hardline);

	return concat(tokens);
}
