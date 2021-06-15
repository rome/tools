/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {createLintVisitor, signals} from "@internal/compiler";
import {jsBlockStatement} from "@internal/ast";
import {descriptions} from "@internal/diagnostics";

export default createLintVisitor({
	name: "js/useSingleCaseStatement",
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
				descriptions.LINT.JS_USE_SINGLE_CASE_STATEMENT,
			);
		}

		return signals.retain;
	},
});
