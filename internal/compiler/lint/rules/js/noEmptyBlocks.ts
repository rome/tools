/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {createLintVisitor, signals} from "@internal/compiler";
import {AnyNode} from "@internal/ast";
import {descriptions} from "@internal/diagnostics";

function isEmpty(node: AnyNode): boolean {
	if (node.innerComments !== undefined && node.innerComments.length > 0) {
		return false;
	}

	if (node.type === "JSEmptyStatement") {
		return true;
	}

	if (node.type === "JSBlockStatement" && node.body.length === 0) {
		return true;
	}

	return false;
}

export default createLintVisitor({
	name: "js/noEmptyBlocks",
	enter(path) {
		const {node, context} = path;

		if (node.type === "JSIfStatement") {
			if (isEmpty(node.consequent)) {
				context.addNodeDiagnostic(
					node.consequent,
					descriptions.LINT.JS_NO_EMPTY_BLOCKS,
				);
			}

			if (node.alternate !== undefined && isEmpty(node.alternate)) {
				context.addNodeDiagnostic(
					node.alternate,
					descriptions.LINT.JS_NO_EMPTY_BLOCKS,
				);
			}
		}

		return signals.retain;
	},
});
