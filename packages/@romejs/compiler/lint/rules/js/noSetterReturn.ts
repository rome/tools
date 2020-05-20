/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from "@romejs/compiler";
import {AnyNode} from "@romejs/ast";
import {getCompletionRecords} from "@romejs/js-ast-utils";
import {descriptions} from "@romejs/diagnostics";

export default {
	name: "noSetterReturn",
	enter(path: Path): AnyNode {
		const {node} = path;

		if (
			(node.type === "JSClassMethod" || node.type === "JSObjectMethod") &&
			node.kind === "set"
		) {
			for (const record of getCompletionRecords(node.body)) {
				if (
					record.type === "COMPLETION" &&
					record.node.type === "JSReturnStatement" &&
					record.node.argument !== undefined
				) {
					path.context.addNodeDiagnostic(
						record.node,
						descriptions.LINT.JS_NO_SETTER_RETURN,
					);
				}
			}
		}

		return node;
	},
};
