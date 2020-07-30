/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Scope from "../Scope";
import {TypeBinding} from "@internal/compiler";
import {AnyNode, tsTypeParameter} from "@internal/ast";
import {createScopeEvaluator} from "./index";

export default createScopeEvaluator({
	inject(node: AnyNode, parent: AnyNode, scope: Scope) {
		node = tsTypeParameter.assert(node);
		scope.addBinding(
			new TypeBinding(
				{
					node,
					name: node.name,
					scope,
				},
				node,
				parent.type === "TSMappedType" ? "mapped type" : "parameter",
			),
		);
	},
});
