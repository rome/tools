/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {createVisitor, signals} from "@internal/compiler";

function isEmpty(arr: undefined | Array<unknown>): boolean {
	return arr === undefined || arr.length === 0;
}

// Transformations can easily put the AST into an invalid state.
//
// This could be setting properties to `undefined` that shouldn't be removed,
// or having an empty array where the AST would expect at least one element.
//
// In an ideal world we would generate runtime validation, however we are
// far from being able to technically do that.
//
// Even if we could we would still need some special handling to decide what
// to do with some of these invalid states to prevent them from needing to be
// handled everywhere.

export default createVisitor({
	name: "clean",
	enter(path) {
		const {node} = path;

		// Remove declarations with no declarators
		if (node.type === "JSVariableDeclaration" && isEmpty(node.declarations)) {
			return signals.remove;
		}

		// Remove declaration statement with no declaration or declarators
		if (
			node.type === "JSVariableDeclarationStatement" &&
			(node.declaration === undefined || isEmpty(node.declaration.declarations))
		) {
			return signals.remove;
		}

		// Remove local named exports with no values
		if (
			node.type === "JSExportLocalDeclaration" &&
			node.declaration === undefined &&
			isEmpty(node.specifiers)
		) {
			return signals.remove;
		}

		// Remove expressionless expression statements
		if (node.type === "JSExpressionStatement" && node.expression === undefined) {
			return signals.remove;
		}

		return signals.retain;
	},
});
