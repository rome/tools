/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from "@romejs/compiler";
import {AnyNode} from "@romejs/ast";
import {doesNodeMatchPattern, getJSXAttribute} from "@romejs/js-ast-utils";
import {descriptions} from "@romejs/diagnostics";

function getJSXChildrenProp(node: AnyNode) {
	return node.type === "JSXElement" && getJSXAttribute(node, "children");
}

function getCreateElementChildrenProp(node: AnyNode) {
	if (
		node.type === "JSCallExpression" &&
		(doesNodeMatchPattern(node.callee, "React.createElement") ||
		doesNodeMatchPattern(node.callee, "createElement")) &&
		node.arguments[1].type === "JSObjectExpression"
	) {
		return node.arguments[1].properties.find((property) =>
			property.type === "JSObjectProperty" &&
			property.key.value.type === "JSIdentifier" &&
			property.key.value.name === "children"
		);
	}
	return undefined;
}

export default {
	name: "reactNoChildrenProp",
	enter(path: Path): AnyNode {
		const {node} = path;
		const childrenProp =
			getJSXChildrenProp(node) || getCreateElementChildrenProp(node);
		if (childrenProp) {
			path.context.addNodeDiagnostic(
				childrenProp,
				descriptions.LINT.REACT_NO_CHILDREN_PROP,
			);
		}

		return node;
	},
};
