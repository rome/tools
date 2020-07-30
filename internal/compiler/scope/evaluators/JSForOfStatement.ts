/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Scope from "../Scope";
import {AnyNode, jsForOfStatement} from "@internal/ast";
import {createScopeEvaluator} from "./index";

export default createScopeEvaluator({
	enter(node: AnyNode, parent: AnyNode, scope: Scope) {
		node = jsForOfStatement.assert(node);
		const newScope = scope.fork("loop", node);
		newScope.injectEvaluate(node.left, node);
		return newScope;
	},
});
