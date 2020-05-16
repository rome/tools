/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode, ImportDeclaration, importDeclaration} from '@romejs/js-ast';
import {Scope} from '../../scopes';
import ImportT from '../../types/ImportT';
import {getImportSpecifiers} from '@romejs/js-ast-utils';

export default function ImportDeclaration(node: AnyNode, scope: Scope) {
	node = importDeclaration.assert(node);

	const source = node.source.value;

	for (const specifier of getImportSpecifiers(node)) {
		if (specifier.type === 'ImportSpecifier') {
			const localName = specifier.local.name.name;
			const importedName = specifier.imported.name;

			const open = new ImportT(
				scope,
				specifier,
				{
					importedName,
					source,
				},
			);
			scope.addBinding(localName, open);
		} else if (specifier.type === 'ImportDefaultSpecifier') {
			const localName = specifier.local.name.name;
			const open = new ImportT(
				scope,
				specifier,
				{
					importedName: 'default',
					source,
				},
			);
			scope.addBinding(localName, open);
		} else if (specifier.type === 'ImportNamespaceSpecifier') {
			const localName = specifier.local.name.name;
			const open = new ImportT(
				scope,
				specifier,
				{
					importedName: undefined,
					source,
				},
			);
			scope.addBinding(localName, open);
		} else {
			// TODO error
		}
	}
}
