/**
* Copyright (c) Facebook, Inc. and its affiliates.
*
* This source code is licensed under the MIT license found in the
* LICENSE file in the root directory of this source tree.
*/

import {Path} from "@romejs/compiler";
import {AnyNode} from "@romejs/ast";
import {descriptions} from "@romejs/diagnostics";

export default {
	name: "reactStylePropObject",
	enter(path: Path): AnyNode {
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
				descriptions.LINT.REACT_STYLE_PROP_OBJECT,
			);
		}

		return node;
	},
};
