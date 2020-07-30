/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyJSDeclaration, AnyNode} from "@internal/ast";

export function isDeclaration(
	node: undefined | AnyNode,
): node is AnyJSDeclaration {
	if (node === undefined) {
		return false;
	}

	switch (node.type) {
		case "JSFunctionDeclaration":
		case "JSClassDeclaration":
		case "JSExportAllDeclaration":
		case "JSExportDefaultDeclaration":
		case "JSExportLocalDeclaration":
		case "JSImportDeclaration":
		case "JSVariableDeclarationStatement":
		case "JSExportExternalDeclaration":
		case "TSDeclareFunction":
		case "TSEnumDeclaration":
		case "TSTypeAlias":
		case "TSExportAssignment":
		case "TSImportEqualsDeclaration":
		case "TSInterfaceDeclaration":
		case "TSModuleDeclaration":
		case "TSNamespaceExportDeclaration": {
			const declaration: AnyJSDeclaration = node;
			declaration;
			return true;
		}

		default: {
			const notDeclaration: Exclude<AnyNode, AnyJSDeclaration> = node;
			notDeclaration;
			return false;
		}
	}
}
