/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Token, concat, space} from '../../tokens';
import {FunctionDeclaration, FunctionExpression} from '@romejs/js-ast';

export default function FunctionExpression(
	builder: Builder,
	node: FunctionDeclaration | FunctionExpression,
): Token {
	const tokens: Array<Token> = [];

	if (node.head.async === true) {
		tokens.push('async');
		tokens.push(space);
	}

	tokens.push('function');

	if (node.head.generator === true) {
		tokens.push('*');
	}

	if (node.id) {
		tokens.push(space, builder.tokenize(node.id, node));
	}

	tokens.push(
		builder.tokenize(node.head, node),
		space,
		builder.tokenize(node.body, node),
	);

	return concat(tokens);
}
