/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from "@romejs/js-compiler";
import {AnyNode} from "@romejs/js-ast";
import {descriptions} from "@romejs/diagnostics";

function isAssignment(path: Path): boolean {
	switch (path.parentPath.node.type) {
		case "AssignmentExpression":
		case "AssignmentArrayPattern":
		case "AssignmentObjectPatternProperty":
		case "UpdateExpression":
		case "AssignmentObjectPattern":
		case "ForInStatement":
			return true;

		default:
			return false;
	}
}

export default {
	name: "noImportAssign",
	enter(path: Path): AnyNode {
		const {node, scope} = path;

		if (
			(node.type === "AssignmentIdentifier" && isAssignment(path)) ||
			(node.type === "ReferenceIdentifier" &&
			path.parentPath.node.type === "UpdateExpression")
		) {
			const binding = scope.getBinding(node.name);
			if (binding !== undefined && binding.kind === "import") {
				path.context.addNodeDiagnostic(
					node,
					descriptions.LINT.NO_IMPORT_ASSIGN(node.name),
				);
			}
		}

		return node;
	},
};
