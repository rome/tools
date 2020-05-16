/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {StaticMemberProperty} from '@romejs/js-ast';
import {Token, concat} from '../../tokens';

export default function StaticMemberProperty(
	builder: Builder,
	node: StaticMemberProperty,
): Token {
	const tokens: Array<Token> = [];

	if (node.optional) {
		tokens.push('?');
	}

	tokens.push('.', builder.tokenize(node.value, node));

	return concat(tokens);
}
