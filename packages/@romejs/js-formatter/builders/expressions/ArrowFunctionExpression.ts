/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ArrowFunctionExpression} from '@romejs/js-ast';
import Builder from '../../Builder';
import {
	Token,
	concat,
	group,
	indent,
	lineOrSpace,
	softline,
	space,
} from '../../tokens';

export default function ArrowFunctionExpression(
	builder: Builder,
	node: ArrowFunctionExpression,
): Token {
	const tokens: Array<Token> = [];

	if (node.head.async === true) {
		tokens.push('async');
		tokens.push(space);
	}

	tokens.push(builder.tokenize(node.head, node));
	tokens.push(space, '=>');

	const body = builder.tokenize(node.body, node);

	// Keep these types of node on the line as the arrow
	if (
		node.body.type === 'ArrayExpression' ||
		node.body.type === 'ObjectExpression' ||
		node.body.type === 'BlockStatement' ||
		node.body.type === 'ArrowFunctionExpression'
	) {
		return group(concat([concat(tokens), space, body]));
	}

	if (node.body.type === 'SequenceExpression') {
		return concat([
			concat(tokens),
			group(concat([space, '(', indent(concat([softline, body])), softline, ')'])),
		]);
	}

	return group(
		concat([
			concat(tokens),
			group(concat([indent(concat([lineOrSpace, body])), softline])),
		]),
	);
}
