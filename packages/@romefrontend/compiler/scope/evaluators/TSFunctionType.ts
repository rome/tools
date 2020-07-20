/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Scope from "../Scope";
import {AnyNode, tsFunctionType} from "@romefrontend/ast";
import {createScopeEvaluator} from "./index";

export default createScopeEvaluator({
	enter(node: AnyNode, parent: AnyNode, scope: Scope) {
		node = tsFunctionType.assert(node);
		const newScope = scope.fork("type-generic", node);
		newScope.injectEvaluate(node.meta.typeParameters, node);
		return newScope;
	},
});
