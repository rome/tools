/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Scope from '../Scope';
import {AnyNode, FunctionHead} from '@romejs/js-ast';
import {ArgumentsBinding, LetBinding} from '../bindings';
import {getBindingIdentifiers} from '@romejs/js-ast-utils';

export default {
	creator: true,
	build(node: FunctionHead, parent: AnyNode, parentScope: Scope) {
		// We already evaluated ourselves
		if (parentScope.node === node) {
			return parentScope;
		}

		const scope = parentScope.fork('function', parent);

		if (parent.type === 'FunctionExpression') {
			const {id} = parent;
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
		scope.evaluate(node.typeParameters, node);

		const params =
			node.rest === undefined ? node.params : [...node.params, node.rest];

		// Add parameters
		for (const param of params) {
			for (const id of getBindingIdentifiers(param)) {
				scope.addBinding(
					new LetBinding({
						node: id,
						name: id.name,
						scope,
						kind: 'parameter',
					}),
				);
			}
		}

		// Add `arguments` binding
		if (parent.type !== 'ArrowFunctionExpression') {
			scope.addBinding(
				new ArgumentsBinding({
					name: 'arguments',
					node,
					scope,
				}),
			);
		}

		return scope;
	},
};
