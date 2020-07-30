/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from "@internal/ast";

export function isTypeNode(node: AnyNode): boolean {
	if (
		node.type.startsWith("Flow") ||
		node.type.startsWith("TS") ||
		node.type.endsWith("TypeAnnotation")
	) {
		return true;
	} else if (node.type === "JSImportDeclaration") {
		return node.importKind === "type" || node.importKind === "typeof";
	} else if (
		node.type === "JSExportDefaultDeclaration" ||
		node.type === "JSExportLocalDeclaration" ||
		node.type === "JSExportAllDeclaration"
	) {
		return node.exportKind === "type";
	} else {
		return false;
	}
}
