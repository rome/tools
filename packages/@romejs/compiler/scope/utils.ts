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
import {AnyJSFunction, JSRoot} from "@romejs/ast";

export function addFunctionBindings(
	scope: Scope,
	node: AnyJSFunction,
	hasArguments: boolean = true,
) {
	const {head} = node;

	// Add type parameters
	scope.evaluate(head.typeParameters);

	const params =
		head.rest === undefined ? head.params : [...head.params, head.rest];

	// Add parameters
	for (const param of params) {
		for (const id of getBindingIdentifiers(param)) {
			// TODO maybe add a `param` binding type?
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
	if (hasArguments) {
		scope.addBinding(
			new ArgumentsBinding({
				name: "arguments",
				node,
				scope,
			}),
		);
	}

	if (head.hasHoistedVars) {
		addVarBindings(scope, node);
	}
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
						scope.evaluate(node, parent);
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
