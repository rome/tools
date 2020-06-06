/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Scope from "./Scope";
import {
	ArgumentsBinding,
	LetBinding,
	REDUCE_SKIP_SUBTREE,
} from "@romejs/compiler";
import {getBindingIdentifiers, isFunctionNode} from "@romejs/js-ast-utils";
import {
	AnyJSFunction,
	JSRoot,
	TSDeclareFunction,
	TSDeclareMethod,
} from "@romejs/ast";

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

export function addVarBindings(scope: Scope, topNode: AnyJSFunction | JSRoot) {
	const {context} = scope.getRootScope();
	scope.setHoistedVars();

	context.reduce(
		topNode,
		[
			{
				name: "scopeVarFunc",
				enter: (path) => {
					const {node, parent} = path;

					if (isFunctionNode(node) && node !== topNode) {
						return REDUCE_SKIP_SUBTREE;
					}

					if (node.type === "JSVariableDeclaration" && node.kind === "var") {
						scope.injectEvaluate(node, parent);
					}

					return node;
				},
			},
		],
		{
			scope,
			noScopeCreation: true,
		},
	);
}
