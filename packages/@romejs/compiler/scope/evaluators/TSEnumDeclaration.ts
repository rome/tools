/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Scope from "../Scope";
import {AnyNode, tsEnumDeclaration} from "@romejs/ast";
import {TypeBinding} from "@romejs/compiler";
import {createScopeEvaluator} from "./index";

export default createScopeEvaluator({
	inject(node: AnyNode, parent: AnyNode, scope: Scope) {
		node = tsEnumDeclaration.assert(node);
		scope.addBinding(
			new TypeBinding(
				{
					node: node.id,
					name: node.id.name,
					scope,
				},
				node,
				"enum",
			),
		);
	},
});
