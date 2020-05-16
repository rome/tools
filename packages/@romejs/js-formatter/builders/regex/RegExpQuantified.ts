/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Token, concat} from '../../tokens';
import {RegExpQuantified} from '@romejs/js-ast';

export default function RegExpQuantified(
	builder: Builder,
	node: RegExpQuantified,
): Token {
	const tokens: Array<Token> = [builder.tokenize(node.target, node)];

	if (node.min === 0 && node.max === 1) {
		tokens.push('?');
	} else if (node.min === 0 && node.max === undefined) {
		tokens.push('*');
	} else if (node.min === 1 && node.max === undefined) {
		tokens.push('+');
	} else {
		tokens.push('{');

		tokens.push(String(node.min));

		if (node.min !== node.max) {
			tokens.push(',');
			if (node.max !== undefined) {
				tokens.push(String(node.max));
			}
		}

		tokens.push('}');
	}

	if (node.lazy) {
		tokens.push('?');
	}

	return concat(tokens);
}
