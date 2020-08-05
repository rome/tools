/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Scope from "./Scope";
import {ArgumentsBinding, LetBinding, signals} from "@internal/compiler";
import {getBindingIdentifiers, isFunctionNode} from "@internal/js-ast-utils";
import {
	AnyJSFunction,
	JSRoot,
	TSDeclareFunction,
	TSDeclareMethod,
} from "@internal/ast";
import {VarBinding} from "./bindings";

export function buildFunctionScope(
	node: AnyJSFunction | TSDeclareFunction | TSDeclareMethod,
	parentScope: Scope,
): Scope {
	const {head} = node;

	const scope = parentScope.fork("function", node);

	if (node.type === "JSFunctionExpression") {
		const {id} = node;
		if (id !== undefined) {
			scope.addBinding(
				new LetBinding({
					node: id,
					name: id.name,
					scope,
				}),
			);
		}
	}

	// Add type parameters
	scope.injectEvaluate(head.typeParameters, head);

	const params =
		head.rest === undefined ? head.params : [...head.params, head.rest];

	// Add parameters
	for (const param of params) {
		for (const id of getBindingIdentifiers(param)) {
			scope.addBinding(
				new LetBinding({
					node: id,
					name: id.name,
					scope,
					kind: "parameter",
				}),
			);
		}
	}

	// Add `arguments` binding
	if (node.type !== "JSArrowFunctionExpression") {
		scope.addBinding(
			new ArgumentsBinding({
				name: "arguments",
				node: head,
				scope,
			}),
		);
	}

	return scope;
}

// var are function scoped so we have separate traversal logic to inject them
export function addVarBindings(scope: Scope, topNode: AnyJSFunction | JSRoot) {
	const {context} = scope.getRootScope();

	context.reduce(
		topNode,
		[
			{
				name: "scopeVarFunc",
				enter: (path) => {
					const {node} = path;

					if (isFunctionNode(node) && node !== topNode) {
						return signals.skip;
					}

					if (node.type === "JSVariableDeclaration" && node.kind === "var") {
						for (const decl of node.declarations) {
							for (const id of getBindingIdentifiers(decl)) {
								scope.addBinding(
									new VarBinding({
										node: id,
										name: id.name,
										scope,
									}),
								);
							}
						}
					}

					return signals.retain;
				},
			},
		],
		{
			scope,
			noScopeCreation: true,
		},
	);
}
