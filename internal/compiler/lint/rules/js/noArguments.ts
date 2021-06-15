/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {createLintVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";

export default createLintVisitor({
	name: "js/noArguments",
	enter(path) {
		const {node, scope} = path;

		if (node.type === "JSReferenceIdentifier" && node.name === "arguments") {
			const args = scope.getBinding("arguments");
			if (args?.kind === "arguments") {
				path.context.addNodeDiagnostic(node, descriptions.LINT.JS_NO_ARGUMENTS);
			}
		}

		return signals.retain;
	},
});
