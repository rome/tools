/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Scope from "../Scope";
import {AnyNode, JSExportDefaultDeclaration} from "@romejs/ast";

export default {
	creator: false,
	build(node: JSExportDefaultDeclaration, parent: AnyNode, scope: Scope) {
		const {declaration} = node;
		const newScope = scope.evaluate(declaration, node);
		if (
			declaration.type === "JSClassDeclaration" ||
			declaration.type === "JSFunctionDeclaration"
		) {
			const id = declaration.id;
			if (id !== undefined) {
				newScope.getBindingAssert(id.name).setExported(true);
			}
		}
		return newScope;
	},
};
