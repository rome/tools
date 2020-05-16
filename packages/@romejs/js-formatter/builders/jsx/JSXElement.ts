/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {
	Token,
	concat,
	group,
	indent,
	join,
	lineOrSpace,
	space,
} from '../../tokens';
import {JSXElement} from '@romejs/js-ast';

export default function JSXElement(builder: Builder, node: JSXElement): Token {
	const tokens: Array<Token> = [
		'<',
		builder.tokenize(node.name, node),
		builder.tokenize(node.typeArguments, node),
	];

	if (node.attributes.length > 0) {
		tokens.push(
			space,
			join(
				lineOrSpace,
				node.attributes.map((attr) => builder.tokenize(attr, node)),
			),
		);
	}

	if (node.selfClosing === true && node.children.length === 0) {
		return group(concat([concat(tokens), space, '/>']));
	} else {
		return concat([
			group(concat([concat(tokens), '>'])),
			indent(concat(node.children.map((child) => builder.tokenize(child, node)))),
			'</',
			builder.tokenize(node.name, node),
			'>',
		]);
	}
}
