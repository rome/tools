/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {createVisitor, signals} from "@romefrontend/compiler";
import {FunctionBinding} from "@romefrontend/compiler/scope/bindings";
import {descriptions} from "@romefrontend/diagnostics";

export default createVisitor({
	name: "js/noFunctionAssign",
	enter(path) {
		const {node, scope} = path;

		if (
			node.type === "JSAssignmentIdentifier" &&
			scope.getBinding(node.name) instanceof FunctionBinding
		) {
			path.context.addNodeDiagnostic(
				node,
				descriptions.LINT.JS_NO_FUNCTION_ASSIGN,
			);
		}

		return signals.retain;
	},
});
