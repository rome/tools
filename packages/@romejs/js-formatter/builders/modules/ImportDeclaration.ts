/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ExportExternalDeclaration, ImportDeclaration} from '@romejs/js-ast';
import Builder from '../../Builder';
import {
	Token,
	concat,
	group,
	ifBreak,
	indent,
	join,
	lineOrSpace,
	softline,
	space,
} from '../../tokens';

export default function ImportDeclaration(
	builder: Builder,
	node: ImportDeclaration,
): Token {
	const tokens: Array<Token> = ['import', space];

	if (node.importKind === 'type' || node.importKind === 'typeof') {
		tokens.push(node.importKind);
		tokens.push(space);
	}

	const {namedSpecifiers, defaultSpecifier, namespaceSpecifier} = node;

	if (
		namedSpecifiers.length > 0 ||
		namespaceSpecifier !== undefined ||
		defaultSpecifier !== undefined
	) {
		tokens.push(printModuleSpecifiers(builder, node), space, 'from', space);
	}

	tokens.push(builder.tokenize(node.source, node), ';');

	return group(concat(tokens));
}

export function printModuleSpecifiers(
	builder: Builder,
	node: ImportDeclaration | ExportExternalDeclaration,
): Token {
	const {namedSpecifiers, defaultSpecifier, namespaceSpecifier} = node;

	const groups: Array<Token> = [];

	if (defaultSpecifier !== undefined) {
		groups.push(builder.tokenize(node.defaultSpecifier, node));
	}

	if (namespaceSpecifier !== undefined) {
		groups.push(builder.tokenize(node.namespaceSpecifier, node));
	}

	if (namedSpecifiers.length > 0) {
		const specifiers: Array<Token> = [];

		for (const specifier of namedSpecifiers) {
			specifiers.push(builder.tokenize(specifier, node));
		}

		if (specifiers.length === 1) {
			// Do not create insert softline tokens when there is a single specifier
			// in order to keep the braces on the same line.
			groups.push(concat(['{', specifiers[0], '}']));
		} else {
			groups.push(
				concat([
					'{',
					indent(concat([softline, join(concat([',', lineOrSpace]), specifiers)])),
					ifBreak(','),
					softline,
					'}',
				]),
			);
		}
	}

	return join(concat([',', space]), groups);
}
