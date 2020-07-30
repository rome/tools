/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Scope from "../Scope";
import {AnyNode, tsInterfaceDeclaration} from "@romefrontend/ast";
import {TypeBinding} from "@romefrontend/compiler";
import {createScopeEvaluator} from "./index";

export default createScopeEvaluator({
	inject(node: AnyNode, parent: AnyNode, scope: Scope) {
		node = tsInterfaceDeclaration.assert(node);
		scope.addBinding(
			new TypeBinding(
				{
					node: node.id,
					name: node.id.name,
					scope,
				},
				node,
				"interface",
			),
		);
	},

	enter(node: AnyNode, parent: AnyNode, scope: Scope) {
		node = tsInterfaceDeclaration.assert(node);
		const newScope = scope.fork("type-generic", node);
		newScope.injectEvaluate(node.typeParameters, node);
		return newScope;
	},
});
