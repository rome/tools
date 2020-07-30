/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {createVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";

export default createVisitor({
	name: "js/noCommaOperator",
	enter(path) {
		const {node} = path;

		if (node.type === "JSSequenceExpression") {
			path.context.addNodeDiagnostic(
				node,
				descriptions.LINT.JS_NO_COMMA_OPERATOR,
			);
		}

		return signals.retain;
	},
});
