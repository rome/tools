/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */
import {Path, createHook} from "@romejs/js-compiler";
import {
	AnyNode,
	FunctionDeclaration,
	ThisExpression,
	VariableDeclarationStatement,
	VariableDeclarator,
	bindingIdentifier,
	blockStatement,
	functionDeclaration,
	returnStatement,
	variableDeclarationStatement,
	variableDeclarator,
} from "@romejs/js-ast";
import {isFunctionNode} from "@romejs/js-ast-utils";
import {descriptions} from "@romejs/diagnostics";

type State = {
	declarators: Array<VariableDeclarator>;
};

type Arg = {
	declarator: VariableDeclarator;
	node: ThisExpression;
};

// This hook is created with a list of initial VariableDeclarators that contain functions we want to convert
// We then remove any ArrowFunctionExpression VariableDeclarators that contain a valid ThisExpression
const hook = createHook<State, Arg, ThisExpression>({
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
		| VariableDeclarationStatement
		| Array<VariableDeclarationStatement | FunctionDeclaration> {
		const node = variableDeclarationStatement.assert(path.node);

		// We may have invalidated all declarations
		if (state.declarators.length === 0) {
			return node;
		}

		const nodes: Array<VariableDeclarationStatement | FunctionDeclaration> = [];

		const newNode: VariableDeclarationStatement = {
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
			const id = bindingIdentifier.assert(decl.id);
			const {init} = decl;

			if (
				init === undefined ||
				(init.type !== "FunctionExpression" &&
				init.type !== "ArrowFunctionExpression")
			) {
				throw new Error("Invalid declarator put into state");
			}

			// TODO if this is suppressed then don't transform
			path.context.addNodeDiagnostic(
				init,
				descriptions.LINT.PREFER_FUNCTION_DECLARATIONS,
				{fixable: true},
			);

			// Convert arrow function body if necessary
			const body =
				init.body.type === "BlockStatement"
					? init.body
					: blockStatement.create({
							body: [returnStatement.quick(init.body)],
						});

			nodes.push(
				functionDeclaration.create({
					id,
					head: init.head,
					body,
				}),
			);
		}

		return nodes;
	},
});

export default {
	name: "preferFunctionDeclarations",
	enter(path: Path): AnyNode {
		const {node} = path;

		if (
			node.type === "VariableDeclarationStatement" &&
			node.declaration.kind === "const"
		) {
			// Get all declarators that are function expressions, have no type annotation, and have a binding identifier id
			const declarators = node.declaration.declarations.filter((decl) => {
				return (
					decl.id.type === "BindingIdentifier" &&
					(decl.id.meta === undefined ||
					decl.id.meta.typeAnnotation === undefined) &&
					decl.init !== undefined &&
					(decl.init.type === "FunctionExpression" ||
					decl.init.type === "ArrowFunctionExpression")
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
		if (node.type === "ThisExpression") {
			// Try to find the arrow function owner, or stop if we get to another function
			const func = path.findAncestry((path) => {
				if (path.node.type === "ArrowFunctionExpression") {
					return path.parent.type === "VariableDeclarator";
				}

				if (isFunctionNode(path.node)) {
					return true;
				}

				return false;
			});

			// We'll only return an ArrowFunctionExpression if it was inside of a VariableDeclarator
			if (func !== undefined && func.node.type === "ArrowFunctionExpression") {
				return path.callHook(
					hook,
					{
						declarator: variableDeclarator.assert(func.parent),
						node,
					},
					node,
				);
			}
		}

		return node;
	},
};
