/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Scope from "../Scope";
import {ImportBinding, ImportBindingMeta} from "@internal/compiler";
import {
	AnyNode,
	ConstJSImportModuleKind,
	jsImportDeclaration,
} from "@internal/ast";
import {getImportSpecifiers} from "@internal/js-ast-utils";
import {createScopeEvaluator} from "./index";

export default createScopeEvaluator({
	inject(node: AnyNode, parent: AnyNode, scope: Scope) {
		node = jsImportDeclaration.assert(node);

		const source = node.source.value;

		for (const specifier of getImportSpecifiers(node)) {
			let kind: ConstJSImportModuleKind = node.importKind || "value";
			let meta: undefined | ImportBindingMeta;

			if (specifier.type === "JSImportNamespaceSpecifier") {
				meta = {
					kind,
					type: "namespace",
					source,
				};
			} else if (specifier.type === "JSImportDefaultSpecifier") {
				meta = {
					kind,
					type: "name",
					imported: "default",
					source,
				};
			} else if (specifier.type === "JSImportSpecifier") {
				meta = {
					kind,
					type: "name",
					imported: specifier.imported.name,
					source,
				};
			}

			if (meta === undefined) {
				return;
			}

			scope.addBinding(
				new ImportBinding(
					{
						node: specifier.local.name,
						name: specifier.local.name.name,
						scope,
					},
					meta,
				),
			);
		}
	},
});
