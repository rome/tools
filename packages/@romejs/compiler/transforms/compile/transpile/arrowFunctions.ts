/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path, createHook} from "@romejs/compiler";
import {
	AnyNode,
	JSIdentifier,
	JSThisExpression,
	jsBindingIdentifier,
	jsIdentifier,
	jsThisExpression,
	jsVariableDeclaration,
	jsVariableDeclarationStatement,
	jsVariableDeclarator,
} from "@romejs/ast";
import {inheritLoc} from "@romejs/js-ast-utils";

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

const arrowProvider = createHook<State, JSThisExpression, JSIdentifier>({
	name: "arrowProvider",
	initialState: {
		id: undefined,
	},
	call(
		path: Path,
		state: State,
		node: JSThisExpression,
	): {
		value: JSIdentifier;
		state: State;
	} {
		const id = state.id === undefined ? path.scope.generateUid() : state.id;
		return {
			value: jsIdentifier.create({
				name: id,
				loc: inheritLoc(node, "this"),
			}),
			state: {
				id,
			},
		};
	},
	exit(path: Path, state: State): AnyNode {
		const {node} = path;

		if (
			node.type !== "JSFunctionDeclaration" &&
			node.type !== "JSFunctionExpression"
		) {
			throw new Error("Only ever expected function nodes");
		}

		// This is called after the subtree has been transformed
		if (state.id === undefined) {
			// No `ThisExpression`s were rewritten
			return node;
		} else {
			return {
				// Inject the binding into the function block
				...node,
				body: {
					...node.body,
					body: [
						jsVariableDeclarationStatement.quick(
							jsVariableDeclaration.create({
								kind: "const",
								declarations: [
									jsVariableDeclarator.create({
										id: jsBindingIdentifier.quick(state.id),
										init: jsThisExpression.create({}),
									}),
								],
							}),
						),
						...node.body.body,
					],
				},
			};
		}
	},
});

export default {
	name: "arrowFunctions",
	enter(path: Path) {
		const {node} = path;

		if (
			node.type === "JSFunctionDeclaration" ||
			node.type === "JSFunctionExpression"
		) {
			// Add a provider to consume `this` inside of arrow functions
			return path.provideHook(arrowProvider);
		}

		if (node.type === "JSThisExpression" && isInsideArrow(path)) {
			// If we're a this expression and we're inside of an arrow then consume us by a descendent provider
			return path.callHook(arrowProvider, node);
		}

		return node;
	},
	exit(path: Path) {
		const {node} = path;

		if (node.type === "JSArrowFunctionExpression") {
			return {
				// Convert all arrow functions into normal functions, we do this in the `exit` method because we
				// still need the arrow to be in the tree for the `isInsideArrow` call in `enter to work
				...node,
				type: "JSFunctionExpression",
			};
		}

		return node;
	},
};
