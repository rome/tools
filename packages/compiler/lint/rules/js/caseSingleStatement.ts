/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {createVisitor, signals} from "@romefrontend/compiler";
import {jsBlockStatement} from "@romefrontend/ast";
import {descriptions} from "@romefrontend/diagnostics";

export default createVisitor({
	name: "js/caseSingleStatement",
	enter(path) {
		const {node} = path;

		if (node.type === "JSSwitchCase" && node.consequent.length > 1) {
			return path.addFixableDiagnostic(
				{
					fixed: signals.replace({
						...node,
						consequent: [jsBlockStatement.quick(node.consequent)],
					}),
				},
				descriptions.LINT.JS_CASE_SINGLE_STATEMENT,
			);
		}

		return signals.retain;
	},
});
