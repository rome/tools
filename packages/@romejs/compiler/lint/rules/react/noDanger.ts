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

function getJSXDangerProp(node: AnyNode) {
	return (
		node.type === "JSXElement" &&
		getJSXAttribute(node, "dangerouslySetInnerHTML")
	);
}

function getCreateElementDangerProp(node: AnyNode) {
	if (
		node.type === "JSCallExpression" &&
		(doesNodeMatchPattern(node.callee, "React.createElement") ||
		doesNodeMatchPattern(node.callee, "createElement")) &&
		node.arguments[1].type === "JSObjectExpression"
	) {
		return node.arguments[1].properties.find((property) =>
			property.type === "JSObjectProperty" &&
			property.key.value.type === "JSIdentifier" &&
			property.key.value.name === "dangerouslySetInnerHTML"
		);
	}
	return undefined;
}

export default {
	name: "reactNoDanger",
	enter(path: Path): AnyNode {
		const {node} = path;
		const dangerProp =
			getJSXDangerProp(node) || getCreateElementDangerProp(node);
		if (dangerProp) {
			path.context.addNodeDiagnostic(
				dangerProp,
				descriptions.LINT.REACT_NO_DANGER,
			);
		}

		return node;
	},
};
