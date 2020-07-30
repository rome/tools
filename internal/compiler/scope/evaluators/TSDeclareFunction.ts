/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Scope from "../Scope";
import {FunctionBinding} from "@romefrontend/compiler";
import {AnyNode, tsDeclareFunction} from "@romefrontend/ast";
import {buildFunctionScope} from "../utils";
import {createScopeEvaluator} from "./index";

export default createScopeEvaluator({
	inject(node: AnyNode, parent: AnyNode, scope: Scope) {
		node = tsDeclareFunction.assert(node);
		scope.addBinding(
			new FunctionBinding({
				node: node.id,
				name: node.id.name,
				scope,
			}),
		);
	},
	enter(node: AnyNode, parent: AnyNode, scope: Scope) {
		return buildFunctionScope(tsDeclareFunction.assert(node), scope);
	},
});
