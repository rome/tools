/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {createLintVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";

export default createLintVisitor({
	name: "js/noTemplateCurlyInString",
	enter(path) {
		const {node, context} = path;

		if (node.type === "JSStringLiteral") {
			const regex = /\$\{[^}]+\}/u;

			if (regex.test(node.value)) {
				context.addNodeDiagnostic(
					node,
					descriptions.LINT.JS_NO_TEMPLATE_CURLY_IN_STRING,
				);
			}
		}

		return signals.retain;
	},
});
