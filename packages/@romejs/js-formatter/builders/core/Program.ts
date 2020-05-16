/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Program} from '@romejs/js-ast';
import {Token, concat, hardline} from '../../tokens';

export default function Program(builder: Builder, node: Program): Token {
	const tokens: Array<Token> = [
		builder.tokenizeStatementList(node.directives, node),
	];

	if (node.directives && node.directives.length) {
		tokens.push(hardline);
	}

	tokens.push(
		builder.tokenizeInnerComments(node, false),
		builder.tokenizeStatementList(node.body, node),
	);

	tokens.push(hardline);

	return concat(tokens);
}
