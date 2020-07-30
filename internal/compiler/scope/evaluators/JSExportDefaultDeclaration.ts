/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Scope from "../Scope";
import {AnyNode, jsExportDefaultDeclaration} from "@internal/ast";
import {createScopeEvaluator} from "./index";

export default createScopeEvaluator({
	inject(node: AnyNode, parent: AnyNode, scope: Scope) {
		node = jsExportDefaultDeclaration.assert(node);

		const {declaration} = node;
		scope.injectEvaluate(declaration, node);

		if (
			declaration.type === "JSClassDeclaration" ||
			declaration.type === "JSFunctionDeclaration"
		) {
			const id = declaration.id;
			if (id !== undefined) {
				scope.getBindingAssert(id.name).setExported(true);
			}
		}
	},
});
