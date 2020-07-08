/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Scope from "../Scope";
import {TypeBinding} from "@romefrontend/compiler";
import {AnyNode, tsTypeAlias} from "@romefrontend/ast";
import {createScopeEvaluator} from "./index";

export default createScopeEvaluator({
	inject(node: AnyNode, parent: AnyNode, scope: Scope) {
		node = tsTypeAlias.assert(node);
		scope.addBinding(
			new TypeBinding(
				{
					node: node.id,
					name: node.id.name,
					scope,
				},
				node,
				"alias",
			),
		);
	},

	enter(node: AnyNode, parent: AnyNode, scope: Scope) {
		node = tsTypeAlias.assert(node);
		const newScope = scope.fork("type-generic", node);
		newScope.injectEvaluate(node.typeParameters);
		return newScope;
	},
});
