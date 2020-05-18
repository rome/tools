/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from "@romejs/js-compiler";
import {AnyNode} from "@romejs/js-ast";
import {descriptions} from "@romejs/diagnostics";

export default {
	name: "noCommaOperator",
	enter(path: Path): AnyNode {
		const {node} = path;

		if (node.type === "SequenceExpression") {
			path.context.addNodeDiagnostic(
				node,
				descriptions.LINT.JAVASCRIPT_NO_COMMA_OPERATOR,
			);
		}

		return node;
	},
};
