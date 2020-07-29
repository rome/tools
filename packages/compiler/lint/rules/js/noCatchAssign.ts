/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {createVisitor, signals} from "@romefrontend/compiler";
import {descriptions} from "@romefrontend/diagnostics";

export default createVisitor({
	name: "js/noCatchAssign",
	enter(path) {
		const {node, context, scope} = path;

		if (node.type === "JSAssignmentIdentifier") {
			const binding = scope.getBinding(node.name);

			if (binding !== undefined && binding.kind === "catch") {
				context.addNodeDiagnostic(node, descriptions.LINT.JS_NO_CATCH_ASSIGN);
			}
		}

		return signals.retain;
	},
});
