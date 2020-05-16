/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSInterfaceDeclaration} from '@romejs/js-ast';
import {Builder} from '@romejs/js-formatter';
import {Token, concat, space} from '../../tokens';
import {printCommaList} from '../utils';

export default function TSInterfaceDeclaration(
	builder: Builder,
	node: TSInterfaceDeclaration,
): Token {
	const tokens: Array<Token> = [];

	if (node.declare) {
		tokens.push('declare', space);
	}

	tokens.push(
		'interface',
		space,
		builder.tokenize(node.id, node),
		builder.tokenize(node.typeParameters, node),
	);

	if (node.extends) {
		tokens.push(
			space,
			'extends',
			space,
			printCommaList(builder, node.extends, node),
		);
	}

	return concat([concat(tokens), space, builder.tokenize(node.body, node)]);
}
