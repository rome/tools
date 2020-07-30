/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Scope from "../Scope";
import {LetBinding} from "@internal/compiler";
import {getBindingIdentifiers} from "@internal/js-ast-utils";
import {AnyNode, jsCatchClause} from "@internal/ast";
import {createScopeEvaluator} from "./index";

export default createScopeEvaluator({
	enter(node: AnyNode, parent: AnyNode, scope: Scope) {
		node = jsCatchClause.assert(node);
		const newScope = scope.fork("block", node);
		if (node.param !== undefined) {
			for (const id of getBindingIdentifiers(node.param)) {
				newScope.addBinding(
					new LetBinding({
						node: id,
						name: id.name,
						scope: newScope,
						kind: "catch",
					}),
				);
			}
		}
		return newScope;
	},
});
