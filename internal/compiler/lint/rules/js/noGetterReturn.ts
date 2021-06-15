/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {createLintVisitor, signals} from "@internal/compiler";
import {getCompletionRecords} from "@internal/js-ast-utils";
import {descriptions} from "@internal/diagnostics";

export default createLintVisitor({
	name: "js/noGetterReturn",
	enter(path) {
		const {node} = path;

		if (
			(node.type === "JSClassMethod" || node.type === "JSObjectMethod") &&
			node.kind === "get"
		) {
			for (const record of getCompletionRecords(node.body)) {
				if (record.type === "INVALID") {
					path.context.addNodeDiagnostic(
						record.node,
						descriptions.LINT.JS_NO_GETTER_RETURN(record.description),
					);
				}
			}
		}

		return signals.retain;
	},
});
