/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Scope from "../Scope";
import {AnyNode, TSImportEqualsDeclaration} from "@romejs/js-ast";
import {ImportBinding} from "@romejs/js-compiler";

export default {
	creator: false,
	build(node: TSImportEqualsDeclaration, parent: AnyNode, scope: Scope) {
		const {moduleReference, id} = node;

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
};
