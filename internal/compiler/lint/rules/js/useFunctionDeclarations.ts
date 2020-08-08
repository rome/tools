/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */
import {createVisitor, signals} from "@internal/compiler";
import {
	JSFunctionDeclaration,
	JSVariableDeclarationStatement,
	JSVariableDeclarator,
	jsBindingIdentifier,
	jsBlockStatement,
	jsFunctionDeclaration,
	jsReturnStatement,
	jsVariableDeclarator,
} from "@internal/ast";
import {isFunctionNode} from "@internal/js-ast-utils";
import {descriptions} from "@internal/diagnostics";

type State = {
	declarators: Array<JSVariableDeclarator>;
};

export default createVisitor<State>({
	name: "js/useFunctionDeclarations",
	enter(path, state) {
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
				state.reset({declarators});
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

				return isFunctionNode(path.node);
			});

			// We'll only return an JSArrowFunctionExpression if it was inside of a JSVariableDeclarator
			if (func !== undefined && func.node.type === "JSArrowFunctionExpression") {
				const declarator = jsVariableDeclarator.assert(func.parent);
				state.set(
					(state) => {
						return {
							declarators: state.declarators.filter((decl) =>
								decl !== declarator
							),
						};
					},
					{
						find: (state) => state.declarators.includes(declarator),
					},
				);
			}
		}

		return signals.retain;
	},

	exit(path, _state) {
		const {node} = path;

		if (node.type === "JSVariableDeclarationStatement" && _state.owns()) {
			const state = _state.get();

			// We may have invalidated all declarations
			if (state.declarators.length === 0) {
				return signals.retain;
			}

			const nodes: Array<JSVariableDeclarationStatement | JSFunctionDeclaration> = [];
			const replaceDeclarators = new Set(state.declarators);

			// Convert functions
			for (const decl of replaceDeclarators) {
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

				const {suppressed} = path.context.addNodeDiagnostic(
					init,
					descriptions.LINT.JS_USE_FUNCTION_DECLARATIONS,
					{tags: {fixable: true}},
				);
				if (suppressed) {
					replaceDeclarators.delete(decl);
					continue;
				}

				// Convert arrow function body if necessary
				const body =
					init.body.type === "JSBlockStatement"
						? init.body
						: jsBlockStatement.create({
								body: [jsReturnStatement.quick(init.body)],
							});

				nodes.push(
					jsFunctionDeclaration.create(
						{
							id,
							head: init.head,
							body,
						},
						init,
					),
				);
			}

			// We may have invalidated all declarations
			if (replaceDeclarators.size === 0) {
				return signals.retain;
			}

			const newNode: JSVariableDeclarationStatement = {
				...node,
				declaration: {
					...node.declaration,
					declarations: node.declaration.declarations.filter((decl) =>
						!replaceDeclarators.has(decl)
					),
				},
			};

			// We may have removed all the declarators
			if (newNode.declaration.declarations.length > 0) {
				nodes.push(newNode);
			}

			if (nodes.length === 1) {
				return signals.replace(nodes[0]);
			}

			return signals.replace(nodes);
		}

		return signals.retain;
	},
});
