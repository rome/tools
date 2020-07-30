/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Scope from "../Scope";
import {AnyNode, tsImportEqualsDeclaration} from "@internal/ast";
import {ImportBinding} from "@internal/compiler";
import {createScopeEvaluator} from "./index";

export default createScopeEvaluator({
	inject(node: AnyNode, parent: AnyNode, scope: Scope) {
		const {moduleReference, id} = tsImportEqualsDeclaration.assert(node);

		if (moduleReference.type === "TSExternalModuleReference") {
			scope.addBinding(
				new ImportBinding(
					{
						node: id,
						name: id.name,
						scope,
					},
					{
						type: "namespace",
						kind: "value",
						source: moduleReference.expression.value,
					},
				),
			);
		} else {
			// TODO
		}
	},
});
