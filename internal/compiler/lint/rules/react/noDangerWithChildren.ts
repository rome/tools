/**
* Copyright (c) Facebook, Inc. and its affiliates.
*
* This source code is licensed under the MIT license found in the
* LICENSE file in the root directory of this source tree.
*/

import {descriptions} from "@internal/diagnostics";
import {AnyNode} from "@internal/ast";
import {Scope, createVisitor, signals} from "@internal/compiler";
import {
	getCreateElementChildren,
	getCreateElementProp,
} from "../../utils/react";

function jsxDangerWithChildren(node: AnyNode) {
	if (node.type !== "JSXElement") {
		return false;
	}

	const hasAttribute = !!node.attributes.find((attribute) =>
		attribute.type === "JSXAttribute" &&
		attribute.name.name === "dangerouslySetInnerHTML"
	);

	return hasAttribute && node.children && node.children.length > 0;
}

function jsxDangerWithPropChildren(node: AnyNode) {
	if (node.type !== "JSXElement") {
		return false;
	}

	const hasDangerAttribute = !!node.attributes.find((attribute) =>
		attribute.type === "JSXAttribute" &&
		attribute.name.name === "dangerouslySetInnerHTML"
	);

	const hasChildrenAttribute = !!node.attributes.find((attribute) =>
		attribute.type === "JSXAttribute" && attribute.name.name === "children"
	);

	return hasDangerAttribute && hasChildrenAttribute;
}

function createElementDangerWithChildren(node: AnyNode, scope: Scope): boolean {
	return (
		!!getCreateElementChildren(node, scope) &&
		!!getCreateElementProp(node, scope, "dangerouslySetInnerHTML")
	);
}

export default createVisitor({
	name: "react/noDangerWithChildren",

	enter(path) {
		const {node, scope} = path;

		if (
			jsxDangerWithChildren(node) ||
			jsxDangerWithPropChildren(node) ||
			createElementDangerWithChildren(node, scope)
		) {
			path.context.addNodeDiagnostic(
				node,
				descriptions.LINT.REACT_NO_DANGER_WITH_CHILDREN,
			);
		}

		return signals.retain;
	},
});
