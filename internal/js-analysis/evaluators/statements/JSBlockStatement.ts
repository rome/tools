/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {AnyNode, JSBlockStatement, jsBlockStatement} from "@internal/ast";
import {getBindingIdentifiers, isTypeNode} from "@internal/js-ast-utils";
import BlockT from "../../types/BlockT";

function shouldHoistExecute(node: undefined | AnyNode): boolean {
	if (node === undefined) {
		return false;
	}

	if (node.type === "JSFunctionDeclaration" || isTypeNode(node)) {
		return true;
	}

	if (
		node.type === "JSExportLocalDeclaration" ||
		node.type === "JSExportDefaultDeclaration"
	) {
		return shouldHoistExecute(node.declaration);
	}

	return false;
}

export default function JSBlockStatement(node: AnyNode, scope: Scope) {
	node = node.type === "JSRoot" ? node : jsBlockStatement.assert(node);

	// Declare variables
	for (const child of node.body) {
		if (child.type === "JSImportDeclaration") {
			scope.evaluate(child);
		}

		const declarations = getBindingIdentifiers(child);
		for (const id of declarations) {
			scope.declareBinding(id.name, id);
		}
	}

	const types = [];

	// Execute hoisted nodes
	const body = [];
	for (const child of node.body) {
		if (child.type === "JSImportDeclaration") {
			// already executed
		} else if (shouldHoistExecute(child)) {
			types.push(scope.evaluate(child));
		} else {
			body.push(child);
		}
	}

	// Execute rest
	for (const child of body) {
		types.push(scope.evaluate(child));
	}

	return new BlockT(scope, node, types);
}
