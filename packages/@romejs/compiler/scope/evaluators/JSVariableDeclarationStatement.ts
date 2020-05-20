/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Scope from "../Scope";
import {AnyNode, JSVariableDeclarationStatement} from "@romejs/ast";
import {getBindingIdentifiers} from "@romejs/js-ast-utils";

export default {
	creator: false,
	build(node: JSVariableDeclarationStatement, parent: AnyNode, scope: Scope) {
		if (node.declare) {
			for (const {name} of getBindingIdentifiers(node)) {
				scope.addGlobal(name);
			}
		} else {
			scope.evaluate(node.declaration, node);
		}
	},
};
