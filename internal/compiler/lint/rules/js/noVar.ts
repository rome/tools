/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {createVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";

export default createVisitor({
	name: "js/noVar",
	enter(path) {
		const {context, node} = path;

		if (node.type === "JSVariableDeclaration" && node.kind === "var") {
			context.addNodeDiagnostic(node, descriptions.LINT.JS_NO_VAR);
		}

		return signals.retain;
	},
});
