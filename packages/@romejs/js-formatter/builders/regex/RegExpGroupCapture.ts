/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Token, concat} from '../../tokens';
import {RegExpGroupCapture} from '@romejs/js-ast';

export default function RegExpGroupCapture(
	builder: Builder,
	node: RegExpGroupCapture,
): Token {
	const tokens: Array<Token> = ['('];

	if (node.name !== undefined) {
		tokens.push('?<');
		tokens.push(node.name);
		tokens.push('>');
	}

	return concat([concat(tokens), builder.tokenize(node.expression, node), ')']);
}
