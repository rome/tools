/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Scope from "../Scope";
import {addVarBindings} from "../utils";
import {AnyNode, jsRoot} from "@internal/ast";
import {createScopeEvaluator} from "./index";

export default createScopeEvaluator({
	enter(node: AnyNode, parent: AnyNode, scope: Scope) {
		node = jsRoot.assert(node);

		const newScope = scope.fork("program", node);

		if (node.hasHoistedVars) {
			addVarBindings(newScope, node);
		}

		for (const child of node.body) {
			newScope.injectEvaluate(child, node);
		}

		return newScope;
	},
});
