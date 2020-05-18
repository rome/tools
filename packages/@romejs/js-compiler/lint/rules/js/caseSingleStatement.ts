/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path, TransformExitResult} from "@romejs/js-compiler";
import {blockStatement} from "@romejs/js-ast";
import {descriptions} from "@romejs/diagnostics";

export default {
	name: "caseSingleStatement",
	enter(path: Path): TransformExitResult {
		const {node, context} = path;

		if (node.type === "SwitchCase" && node.consequent.length > 1) {
			return context.addFixableDiagnostic(
				{
					old: node,
					fixed: {
						...node,
						consequent: [blockStatement.quick(node.consequent)],
					},
				},
				descriptions.LINT.JS_CASE_SINGLE_STATEMENT,
			);
		}

		return node;
	},
};
