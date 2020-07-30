/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Scope from "../Scope";
import {AnyNode, jsExportLocalDeclaration} from "@internal/ast";
import {getBindingIdentifiers} from "@internal/js-ast-utils";
import {createScopeEvaluator} from "./index";

export default createScopeEvaluator({
	inject(node: AnyNode, parent: AnyNode, scope: Scope) {
		node = jsExportLocalDeclaration.assert(node);
		scope.injectEvaluate(node.declaration, node);
		for (const id of getBindingIdentifiers(node)) {
			const binding = scope.getBinding(id.name);
			if (binding !== undefined) {
				binding.setExported(true);
			}
		}
	},
});
