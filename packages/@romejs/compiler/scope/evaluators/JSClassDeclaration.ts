/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Scope from "../Scope";
import {ClassBinding} from "@romejs/compiler";
import {AnyNode, jsClassDeclaration} from "@romejs/ast";
import JSClassExpression from "./JSClassExpression";
import {createScopeEvaluator} from "./index";

export default createScopeEvaluator({
	inject(node: AnyNode, parent: AnyNode, scope: Scope) {
		node = jsClassDeclaration.assert(node);
		if (node.id !== undefined) {
			scope.addBinding(
				new ClassBinding({
					name: node.id.name,
					node: node.id,
					scope,
				}),
			);
		}
	},
	enter(node: AnyNode, parent: AnyNode, scope: Scope) {
		return JSClassExpression.enter(node, parent, scope);
	},
});
