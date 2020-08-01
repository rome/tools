/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path, createVisitor, signals} from "@internal/compiler";
import {
	AnyNode,
	JSFunctionDeclaration,
	JSFunctionExpression,
	JSRoot,
	jsBindingIdentifier,
	jsBlockStatement,
	jsReferenceIdentifier,
	jsReturnStatement,
	jsThisExpression,
	jsVariableDeclaration,
	jsVariableDeclarationStatement,
	jsVariableDeclarator,
} from "@internal/ast";

function isInsideArrow(path: Path): boolean {
	for (const ancestor of path.ancestryPaths) {
		const {type} = ancestor.node;

		// If we hit a function first then it takes precedence over any arrow

		// NOTE: There are other nodes for functions not included
		if (type === "JSFunctionExpression" || type === "JSFunctionDeclaration") {
			return false;
		}

		if (type === "JSArrowFunctionExpression") {
			return true;
		}
	}

	return false;
}

type State = {
	id: undefined | string;
};

function isArrowTarget(
	node: AnyNode,
): node is JSFunctionDeclaration | JSFunctionExpression | JSRoot {
	return (
		node.type === "JSFunctionDeclaration" ||
		node.type === "JSFunctionExpression" ||
		node.type === "JSRoot"
	);
}

export default createVisitor<State>({
	name: "arrowFunctions",

	enter(path, state) {
		const {node} = path;

		if (isArrowTarget(node)) {
			state.reset({
				id: undefined,
			});
		}

		if (node.type === "JSThisExpression" && isInsideArrow(path)) {
			let id = state.get().id;
			if (id === undefined) {
				id = path.scope.generateUid();
				state.set({id});
			}
			return signals.replace(jsReferenceIdentifier.quick(id));
		}

		return signals.retain;
	},

	exit(path, state) {
		const {node} = path;

		if (node.type === "JSArrowFunctionExpression") {
			const newNode: JSFunctionExpression = {
				// Convert all arrow functions into normal functions, we do this in the `exit` method because we
				// still need the arrow to be in the tree for the `isInsideArrow` call in `enter to work
				...node,
				type: "JSFunctionExpression",
				body: node.body.type === "JSBlockStatement"
					? node.body
					: jsBlockStatement.quick([jsReturnStatement.create(node.body)]),
			};
			return signals.replace(newNode);
		}

		if (isArrowTarget(node) && state.owns()) {
			const id = state.get().id;

			if (id !== undefined) {
				const decl = jsVariableDeclarationStatement.quick(
					jsVariableDeclaration.create({
						kind: "const",
						declarations: [
							jsVariableDeclarator.create({
								id: jsBindingIdentifier.quick(id),
								init: jsThisExpression.create({}),
							}),
						],
					}),
				);

				if (node.type === "JSRoot") {
					return signals.replace({
						...node,
						body: [decl, ...node.body],
					});
				} else {
					// Inject the binding into the function block
					return signals.replace({
						...node,
						body: {
							...node.body,
							body: [decl, ...node.body.body],
						},
					});
				}
			}
		}

		return signals.retain;
	},
});
