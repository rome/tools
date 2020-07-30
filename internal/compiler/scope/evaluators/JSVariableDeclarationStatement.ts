/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Scope from "../Scope";
import {AnyNode, jsVariableDeclarationStatement} from "@internal/ast";
import {getBindingIdentifiers} from "@internal/js-ast-utils";
import {createScopeEvaluator} from "./index";

export default createScopeEvaluator({
	inject(node: AnyNode, parent: AnyNode, scope: Scope) {
		node = jsVariableDeclarationStatement.assert(node);

		if (node.declare) {
			for (const {name} of getBindingIdentifiers(node)) {
				scope.addGlobal(name);
			}
		} else {
			scope.injectEvaluate(node.declaration, node);
		}
	},
});
