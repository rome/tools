/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {createVisitor, signals} from "@romefrontend/compiler";
import {descriptions} from "@romefrontend/diagnostics";

export default createVisitor({
	name: "js/noArguments",
	enter(path) {
		const {node, scope} = path;

		if (node.type === "JSReferenceIdentifier" && node.name === "arguments") {
			const args = scope.getBinding("arguments");
			if (args && args.kind === "arguments") {
				path.context.addNodeDiagnostic(node, descriptions.LINT.JS_NO_ARGUMENTS);
			}
		}

		return signals.retain;
	},
});
