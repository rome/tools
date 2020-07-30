/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Scope from "../Scope";
import {AnyNode, tsMappedType} from "@internal/ast";
import {createScopeEvaluator} from "./index";

export default createScopeEvaluator({
	enter(node: AnyNode, parent: AnyNode, scope: Scope) {
		node = tsMappedType.assert(node);
		const newScope = scope.fork("type-generic", node);
		newScope.injectEvaluate(node.typeParameter, node);
		return newScope;
	},
});
