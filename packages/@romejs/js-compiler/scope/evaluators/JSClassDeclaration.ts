/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Scope from "../Scope";
import {ClassBinding} from "@romejs/js-compiler";
import {AnyNode, JSClassDeclaration} from "@romejs/ast";
import JSClassExpression from "./JSClassExpression";

export default {
	creator: false,
	build(node: JSClassDeclaration, parent: AnyNode, scope: Scope) {
		if (node.id !== undefined) {
			scope.addBinding(
				new ClassBinding({
					name: node.id.name,
					node: node.id,
					scope,
				}),
			);
		}
		return JSClassExpression.build(node, parent, scope);
	},
};
