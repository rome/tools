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
} from "@romejs/ast";
import {Path} from "@romejs/js-compiler";
import {UnknownFilePath} from "@romejs/path";
import {renameBindings} from "@romejs/js-ast-utils";
import {TransformExitResult} from "@romejs/js-compiler/types";
import {descriptions} from "@romejs/diagnostics";
import {toVariableCamelCase} from "./camelCase";

function isValidDeclaration(
	node: AnyNode,
): node is JSFunctionDeclaration | JSClassDeclaration {
	return (
		node.type === "JSFunctionDeclaration" || node.type === "JSClassDeclaration"
	);
}

export function filenameToId(
	path: UnknownFilePath,
	capitalize: boolean,
): undefined | string {
	let basename = path.getExtensionlessBasename();

	if (basename === "index") {
		// If the filename is `index` then use the parent directory name
		basename = path.getParent().getExtensionlessBasename();
	}

	return toVariableCamelCase(basename, capitalize);
}

export default {
	name: "defaultExportSameBasename",
	enter(path: Path): TransformExitResult {
		const {context, node} = path;

		if (node.type === "JSProgram") {
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

						return context.addFixableDiagnostic(
							{
								target: id,
								old: node,
								fixed: renameBindings(path, new Map([[id.name, basename]])),
							},
							descriptions.LINT.JS_DEFAULT_EXPORT_SAME_BASENAME({
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

		return node;
	},
};
