/**
* Copyright (c) Facebook, Inc. and its affiliates.
*
* This source code is licensed under the MIT license found in the
* LICENSE file in the root directory of this source tree.
*/

import {createLintVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";

export default createLintVisitor({
	name: "react/useStylePropObject",
	enter(path) {
		const {node} = path;

		if (
			node.type === "JSXAttribute" &&
			node.name.name === "style" &&
			node.value !== undefined &&
			((node.value.type === "JSXExpressionContainer" &&
			node.value.expression.type !== "JSObjectExpression") ||
			node.value.type !== "JSXExpressionContainer")
		) {
			path.context.addNodeDiagnostic(
				node,
				descriptions.LINT.REACT_USE_STYLE_PROP_OBJECT,
			);
		}

		return signals.retain;
	},
});
