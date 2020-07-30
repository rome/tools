/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {AnyNode, JSLogicalExpression, jsLogicalExpression} from "@internal/ast";
import T from "../../types/T";
import UnionT from "../../types/UnionT";

function uniq(args: Array<string>): Array<string> {
	return [...new Set(args)];
}

export default function JSLogicalExpression(node: AnyNode, scope: Scope) {
	node = jsLogicalExpression.assert(node);

	switch (node.operator) {
		case "||": {
			const left = scope.refine().evaluate(node.left);
			const right = scope.refine().evaluate(node.right);

			// create a new scope that has unions of all the refined bindings
			const refinedScope = scope.refine();
			const refinedNames = uniq([
				...left.scope.getOwnBindingNames(),
				...right.scope.getOwnBindingNames(),
			]);
			const mergeScopes = [left.scope, right.scope];
			for (const name of refinedNames) {
				const rawTypes: Set<T> = new Set();
				for (const scope of mergeScopes) {
					const binding = scope.getBinding(name);
					if (binding !== undefined) {
						rawTypes.add(binding);
					}
				}

				const types = Array.from(rawTypes);
				refinedScope.addBinding(name, refinedScope.createUnion(types));
			}

			return new UnionT(refinedScope, node, [left, right]);
		}

		case "&&": {
			const left = scope.evaluate(node.left);
			const right = left.scope.evaluate(node.right);
			return new UnionT(right.scope, node, [left, right]);
		}

		default:
			throw new Error("Unknown operator");
	}
}
