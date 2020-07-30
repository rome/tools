/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Scope from "../Scope";
import {AnyNode, tsTypeParameterDeclaration} from "@internal/ast";
import {createScopeEvaluator} from "./index";

export default createScopeEvaluator({
	inject(node: AnyNode, parent: AnyNode, scope: Scope) {
		node = tsTypeParameterDeclaration.assert(node);
		for (const param of node.params) {
			scope.injectEvaluate(param, node);
		}
	},
});
