/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Scope from "../Scope";
import {AnyNode, jsBlockStatement} from "@internal/ast";
import {isFunctionNode} from "@internal/js-ast-utils";
import {addVarBindings} from "../utils";
import {createScopeEvaluator} from "./index";

export default createScopeEvaluator({
	enter(node: AnyNode, parent: AnyNode, scope: Scope) {
		node = jsBlockStatement.assert(node);
		const newScope = scope.fork("block", node);

		if (isFunctionNode(parent) && parent.head.hasHoistedVars) {
			addVarBindings(newScope, parent);
		}

		for (const child of node.body) {
			newScope.injectEvaluate(child, node);
		}

		return newScope;
	},
});
