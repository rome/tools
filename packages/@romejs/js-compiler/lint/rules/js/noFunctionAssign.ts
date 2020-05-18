/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from "@romejs/js-compiler";
import {AnyNode} from "@romejs/js-ast";
import {FunctionBinding} from "@romejs/js-compiler/scope/bindings";
import {descriptions} from "@romejs/diagnostics";

export default {
	name: "noFunctionAssign",
	enter(path: Path): AnyNode {
		const {node, scope} = path;

		if (
			node.type === "AssignmentIdentifier" &&
			scope.getBinding(node.name) instanceof FunctionBinding
		) {
			path.context.addNodeDiagnostic(
				node,
				descriptions.LINT.JS_NO_FUNCTION_ASSIGN,
			);
		}

		return node;
	},
};
