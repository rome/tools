/**
* Copyright (c) Facebook, Inc. and its affiliates.
*
* This source code is licensed under the MIT license found in the
* LICENSE file in the root directory of this source tree.
*/

import {descriptions} from "@romejs/diagnostics";
import {AnyNode, ObjectExpression} from "@romejs/js-ast";
import {Path} from "@romejs/js-compiler";
import {doesNodeMatchPattern} from "@romejs/js-ast-utils";

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

function createElementDangerWithChildren(node: AnyNode): boolean {
	if (node.type !== "CallExpression") {
		return false;
	}

	const propsArgument = node.arguments[node.arguments.length - 2];

	return (
		doesNodeMatchPattern(node.callee, "React.createElement") &&
		node.arguments.length === 3 &&
		propsArgument.type === "ObjectExpression" &&
		propsArgument.properties.some((prop) =>
			prop.type === "ObjectProperty" &&
			prop.key.type === "StaticPropertyKey" &&
			prop.key.value.type === "Identifier" &&
			prop.key.value.name === "dangerouslySetInnerHTML"
		)
	);
}

function createElementDangerWithPropChildren(node: AnyNode): boolean {
	if (node.type !== "CallExpression") {
		return false;
	}

	const propsArgument = node.arguments[1];

	function hasDangerAttribute(node: ObjectExpression) {
		return node.properties.some((prop) =>
			prop.type === "ObjectProperty" &&
			prop.key.type === "StaticPropertyKey" &&
			prop.key.value.type === "Identifier" &&
			prop.key.value.name === "dangerouslySetInnerHTML"
		);
	}

	function hasChildrenAttribute(node: ObjectExpression) {
		return node.properties.some((prop) =>
			prop.type === "ObjectProperty" &&
			prop.key.type === "StaticPropertyKey" &&
			prop.key.value.type === "Identifier" &&
			prop.key.value.name === "children"
		);
	}

	return (
		doesNodeMatchPattern(node.callee, "React.createElement") &&
		propsArgument.type === "ObjectExpression" &&
		hasDangerAttribute(propsArgument) &&
		hasChildrenAttribute(propsArgument)
	);
}

export default {
	name: "noDangerWithChildren",

	enter(path: Path): AnyNode {
		const {node} = path;

		if (
			jsxDangerWithChildren(node) ||
			jsxDangerWithPropChildren(node) ||
			createElementDangerWithChildren(node) ||
			createElementDangerWithPropChildren(node)
		) {
			path.context.addNodeDiagnostic(
				node,
				descriptions.LINT.REACT_NO_DANGER_WITH_CHILDREN,
			);
		}

		return node;
	},
};
