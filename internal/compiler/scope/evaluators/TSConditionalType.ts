/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Scope from "../Scope";
import {AnyNode} from "@internal/ast";
import {createScopeEvaluator} from "./index";
import {signals} from "@internal/compiler";

export default createScopeEvaluator({
	enter(node: AnyNode, parent: AnyNode, scope: Scope) {
		let topNode = node;
		const newScope = scope.fork("conditional-type", node);
		const {
			context,
		} = scope.getRootScope();

		context.reduce(
			topNode,
			[
				{
					name: "inferInject",
					enter: (path) => {
						const {
							node,
						} = path;

						if (node.type === "TSConditionalType" && node !== topNode) {
							return signals.skip;
						}

						if (node.type === "TSInferType") {
							scope.injectEvaluate(node.typeParameter, node);
						}

						return signals.retain;
					},
				},
			],
			{
				scope,
				noScopeCreation: true,
			},
		);

		return newScope;
	},
});
