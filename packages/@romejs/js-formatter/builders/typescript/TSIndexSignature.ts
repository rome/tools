/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSIndexSignature} from '@romejs/js-ast';
import {Builder} from '@romejs/js-formatter';
import {Token, concat, space} from '../../tokens';

export default function TSIndexSignature(
	builder: Builder,
	node: TSIndexSignature,
): Token {
	const tokens: Array<Token> = [];

	if (node.readonly) {
		tokens.push('readonly');
		tokens.push(space);
	}

	return concat([
		concat(tokens),
		'[',
		builder.tokenize(node.key, node),
		']',
		':',
		space,
		builder.tokenize(node.typeAnnotation, node),
		';',
	]);
}
