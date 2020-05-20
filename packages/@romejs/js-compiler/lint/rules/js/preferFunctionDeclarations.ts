/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */
import {Path, createHook} from "@romejs/js-compiler";
import {
	AnyNode,
	JSFunctionDeclaration,
	JSThisExpression,
	JSVariableDeclarationStatement,
	JSVariableDeclarator,
	jsBindingIdentifier,
	jsBlockStatement,
	jsFunctionDeclaration,
	jsReturnStatement,
	jsVariableDeclarationStatement,
	jsVariableDeclarator,
} from "@romejs/ast";
import {isFunctionNode} from "@romejs/js-ast-utils";
import {descriptions} from "@romejs/diagnostics";

type State = {
	declarators: Array<JSVariableDeclarator>;
};

type Arg = {
	declarator: JSVariableDeclarator;
	node: JSThisExpression;
};

// This hook is created with a list of initial JSVariableDeclarators that contain functions we want to convert
// We then remove any JSArrowFunctionExpression JSVariableDeclarators that contain a valid JSThisExpression
const hook = createHook<State, Arg, JSThisExpression>({
	name: "preferFunctionDeclarationsHook",
	initialState: {
		declarators: [],
	},
	call(path: Path, state: State, {declarator, node}: Arg) {
		return {
			bubble: !state.declarators.includes(declarator),
			value: node,
			state: {
				declarators: state.declarators.filter((decl) => decl !== declarator),
			},
		};
	},
	exit(
		path: Path,
		state: State,
	):
		| JSVariableDeclarationStatement
		| JSFunctionDeclaration
		| Array<JSVariableDeclarationStatement | JSFunctionDeclaration> {
		const node = jsVariableDeclarationStatement.assert(path.node);

		// We may have invalidated all declarations
		if (state.declarators.length === 0) {
			return node;
		}

		const nodes: Array<JSVariableDeclarationStatement | JSFunctionDeclaration> = [];

		const newNode: JSVariableDeclarationStatement = {
			...node,
			declaration: {
				...node.declaration,
				declarations: node.declaration.declarations.filter((decl) =>
					!state.declarators.includes(decl)
				),
			},
		};

		// We may have removed all the declarators
		if (newNode.declaration.declarations.length > 0) {
			nodes.push(newNode);
		}

		// Convert functions
		for (const decl of state.declarators) {
			// Could have been changed under us. Ignore it, we'll get it in another pass
			if (!node.declaration.declarations.includes(decl)) {
				continue;
			}

			const id = jsBindingIdentifier.assert(decl.id);
			const {init} = decl;

			if (
				init === undefined ||
				(init.type !== "JSFunctionExpression" &&
				init.type !== "JSArrowFunctionExpression")
			) {
				throw new Error("Invalid declarator put into state");
			}

			// TODO if this is suppressed then don't transform
			path.context.addNodeDiagnostic(
				init,
				descriptions.LINT.JS_PREFER_FUNCTION_DECLARATIONS,
				{fixable: true},
			);

			// Convert arrow function body if necessary
			const body =
				init.body.type === "JSBlockStatement"
					? init.body
					: jsBlockStatement.create({
							body: [jsReturnStatement.quick(init.body)],
						});

			nodes.push(
				jsFunctionDeclaration.create({
					id,
					head: init.head,
					body,
				}),
			);
		}

		if (nodes.length === 1) {
			return nodes[0];
		}

		return nodes;
	},
});

export default {
	name: "preferFunctionDeclarations",
	enter(path: Path): AnyNode {
		const {node} = path;

		if (
			node.type === "JSVariableDeclarationStatement" &&
			node.declaration.kind === "const"
		) {
			// Get all declarators that are function expressions, have no type annotation, and have a binding jsIdentifier id
			const declarators = node.declaration.declarations.filter((decl) => {
				return (
					decl.id.type === "JSBindingIdentifier" &&
					(decl.id.meta === undefined ||
					decl.id.meta.typeAnnotation === undefined) &&
					decl.init !== undefined &&
					(decl.init.type === "JSFunctionExpression" ||
					decl.init.type === "JSArrowFunctionExpression")
				);
			});
			if (declarators.length > 0) {
				return path.provideHook(
					hook,
					{
						declarators,
					},
				);
			}
		}

		// If we have a `this` inside of an arrow function attached as a variable declarator then we should consider
		// it valid
		if (node.type === "JSThisExpression") {
			// Try to find the arrow function owner, or stop if we get to another function
			const func = path.findAncestry((path) => {
				if (path.node.type === "JSArrowFunctionExpression") {
					return path.parent.type === "JSVariableDeclarator";
				}

				if (isFunctionNode(path.node)) {
					return true;
				}

				return false;
			});

			// We'll only return an JSArrowFunctionExpression if it was inside of a JSVariableDeclarator
			if (func !== undefined && func.node.type === "JSArrowFunctionExpression") {
				return path.callHook(
					hook,
					{
						declarator: jsVariableDeclarator.assert(func.parent),
						node,
					},
					node,
				);
			}
		}

		return node;
	},
};
