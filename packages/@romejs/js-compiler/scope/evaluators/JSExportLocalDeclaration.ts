/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Scope from "../Scope";
import {AnyNode, JSExportLocalDeclaration} from "@romejs/ast";
import {getBindingIdentifiers} from "@romejs/js-ast-utils";

export default {
	creator: false,
	build(node: JSExportLocalDeclaration, parent: AnyNode, scope: Scope) {
		const newScope = scope.evaluate(node.declaration, node);
		for (const id of getBindingIdentifiers(node)) {
			newScope.getBindingAssert(id.name).setExported(true);
		}
		return newScope;
	},
};
