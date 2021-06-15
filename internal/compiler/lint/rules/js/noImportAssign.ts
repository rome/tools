/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {CompilerPath, createLintVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";

function isAssignment(path: CompilerPath): boolean {
	switch (path.parentPath.node.type) {
		case "JSAssignmentExpression":
		case "JSAssignmentArrayPattern":
		case "JSAssignmentObjectPatternProperty":
		case "JSUpdateExpression":
		case "JSAssignmentObjectPattern":
		case "JSForInStatement":
			return true;

		default:
			return false;
	}
}

export default createLintVisitor({
	name: "js/noImportAssign",
	enter(path) {
		const {node, scope} = path;

		if (
			(node.type === "JSAssignmentIdentifier" && isAssignment(path)) ||
			(node.type === "JSReferenceIdentifier" &&
			path.parentPath.node.type === "JSUpdateExpression")
		) {
			const binding = scope.getBinding(node.name);
			if (binding?.kind === "import") {
				path.context.addNodeDiagnostic(
					node,
					descriptions.LINT.JS_NO_IMPORT_ASSIGN(node.name),
				);
			}
		}

		return signals.retain;
	},
});
