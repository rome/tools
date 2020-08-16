/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyNode,
	JSClassDeclaration,
	JSExportDefaultDeclaration,
	JSFunctionDeclaration,
} from "@internal/ast";
import {createVisitor, signals} from "@internal/compiler";
import {UnknownPath} from "@internal/path";
import {renameBindings} from "@internal/js-ast-utils";
import {descriptions} from "@internal/diagnostics";
import {normalizeCamelCase} from "./useCamelCase";
import {toCamelCase} from "@internal/string-utils";

function isValidDeclaration(
	node: AnyNode,
): node is JSFunctionDeclaration | JSClassDeclaration {
	return (
		node.type === "JSFunctionDeclaration" || node.type === "JSClassDeclaration"
	);
}

export function filenameToId(
	path: UnknownPath,
	capitalize: boolean,
): undefined | string {
	let basename = path.getExtensionlessBasename();

	if (basename === "index") {
		if (!path.hasParent()) {
			return undefined;
		}

		// If the filename is `index` then use the parent directory name
		basename = path.getParent().getExtensionlessBasename();
	}

	return normalizeCamelCase(
		toCamelCase(
			basename,
			{
				forcePascal: capitalize,
				allowShouty: true,
			},
		),
	);
}

export default createVisitor({
	name: "js/useDefaultExportBasename",
	enter(path) {
		const {context, node} = path;

		if (node.type === "JSRoot") {
			let defaultExport: undefined | JSExportDefaultDeclaration;
			for (const bodyNode of node.body) {
				if (bodyNode.type === "JSExportDefaultDeclaration") {
					defaultExport = bodyNode;
					break;
				}
			}

			if (
				defaultExport !== undefined &&
				isValidDeclaration(defaultExport.declaration)
			) {
				const {declaration} = defaultExport;

				// Get the export default id
				const id = declaration.id;
				if (id !== undefined && context.path !== undefined) {
					const type =
						declaration.type === "JSFunctionDeclaration" ? "function" : "class";
					const basename = filenameToId(context.path, type === "class");

					if (basename !== undefined && basename !== id.name) {
						const correctFilename = id.name + context.path.getExtensions();

						return path.addFixableDiagnostic(
							{
								target: id,
								fixed: signals.replace(
									renameBindings(path, new Map([[id.name, basename]])),
								),
							},
							descriptions.LINT.JS_USE_DEFAULT_EXPORT_BASENAME({
								defaultName: id.name,
								defaultType: type,
								actualFilename: basename,
								correctFilename,
							}),
						);
					}
				}
			}
		}

		return signals.retain;
	},
});
