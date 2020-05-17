/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {AnyNode, BlockStatement, blockStatement} from "@romejs/js-ast";
import {getBindingIdentifiers, isTypeNode} from "@romejs/js-ast-utils";
import BlockT from "../../types/BlockT";

function shouldHoistExecute(node: undefined | AnyNode): boolean {
	if (node === undefined) {
		return false;
	}

	if (node.type === "FunctionDeclaration" || isTypeNode(node)) {
		return true;
	}

	if (
		node.type === "ExportLocalDeclaration" ||
		node.type === "ExportDefaultDeclaration"
	) {
		return shouldHoistExecute(node.declaration);
	}

	return false;
}

export default function BlockStatement(node: AnyNode, scope: Scope) {
	node = node.type === "Program" ? node : blockStatement.assert(node);

	// Declare variables
	for (const child of node.body) {
		if (child.type === "ImportDeclaration") {
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
		if (child.type === "ImportDeclaration") {
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
