/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Scope from "../Scope";
import {FunctionBinding} from "@romejs/js-compiler";
import {AnyNode, JSFunctionDeclaration} from "@romejs/ast";

export default {
	creator: false,
	build(node: JSFunctionDeclaration, parent: AnyNode, scope: Scope) {
		if (node.id !== undefined) {
			scope.addBinding(
				new FunctionBinding({
					node: node.id,
					name: node.id.name,
					scope,
				}),
			);
		}
	},
};
