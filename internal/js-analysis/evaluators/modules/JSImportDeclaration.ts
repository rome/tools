/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode, JSImportDeclaration, jsImportDeclaration} from "@internal/ast";
import {Scope} from "../../scopes";
import ImportT from "../../types/ImportT";
import {getImportSpecifiers} from "@internal/js-ast-utils";

export default function JSImportDeclaration(node: AnyNode, scope: Scope) {
	node = jsImportDeclaration.assert(node);

	const source = node.source.value;

	for (const specifier of getImportSpecifiers(node)) {
		if (specifier.type === "JSImportSpecifier") {
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
		} else if (specifier.type === "JSImportDefaultSpecifier") {
			const localName = specifier.local.name.name;
			const open = new ImportT(
				scope,
				specifier,
				{
					importedName: "default",
					source,
				},
			);
			scope.addBinding(localName, open);
		} else if (specifier.type === "JSImportNamespaceSpecifier") {
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
