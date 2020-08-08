/**
* Copyright (c) Facebook, Inc. and its affiliates.
*
* This source code is licensed under the MIT license found in the
* LICENSE file in the root directory of this source tree.
*/

import {createVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";
import {JSStringLiteral} from "@internal/ast";
import {
	getCreateElementChildren,
	getCreateElementProp,
	getCreateElementType,
	isCreateElement,
} from "../../utils/react";
import {VOID_DOM_ELEMENTS} from "../../utils/constants";

export default createVisitor({
	name: "react/noVoidElementsWithChildren",
	enter(path) {
		const {node, scope} = path;
		const elementType = getCreateElementType(node, scope);

		if (
			isCreateElement(node, scope) &&
			elementType &&
			VOID_DOM_ELEMENTS.has(elementType)
		) {
			const childrenNode = getCreateElementChildren(node, scope);
			const dangerNode = getCreateElementProp(
				node,
				scope,
				"dangerouslySetInnerHTML",
			);

			if (Array.isArray(childrenNode)) {
				return path.addFixableDiagnostic(
					{
						target: node.arguments,
						fixed: signals.replace({
							...node,
							arguments: [node.arguments[0], node.arguments[1]],
						}),
					},
					descriptions.LINT.REACT_NO_VOID_ELEMENTS_WITH_CHILDREN(
						(node.arguments[0] as JSStringLiteral).value,
						["children"],
					),
				);
			}

			if (elementType && childrenNode) {
				return path.addFixableDiagnostic(
					{
						target: childrenNode,
						fixed: signals.remove,
					},
					descriptions.LINT.REACT_NO_VOID_ELEMENTS_WITH_CHILDREN(
						elementType,
						["children"],
					),
				);
			}

			if (elementType && dangerNode) {
				return path.addFixableDiagnostic(
					{
						target: dangerNode,
						fixed: signals.remove,
					},
					descriptions.LINT.REACT_NO_VOID_ELEMENTS_WITH_CHILDREN(
						elementType,
						["dangerouslySetInnerHTML"],
					),
				);
			}
		}

		if (
			node.type === "JSXElement" &&
			node.name.type === "JSXIdentifier" &&
			VOID_DOM_ELEMENTS.has(node.name.name)
		) {
			const element = node.name.name;

			let properties: Set<string> = new Set();

			if (node.children.length !== 0) {
				properties.add("children");
			}

			const newAttributes = [];
			for (const attribute of node.attributes) {
				if (attribute.type === "JSXAttribute") {
					const property = attribute.name.name;
					if (property === "children") {
						properties.add("children");
						continue;
					} else if (property === "dangerouslySetInnerHTML") {
						properties.add("dangerouslySetInnerHTML");
						continue;
					}
				}
				newAttributes.push(attribute);
			}

			if (properties.size > 0) {
				return path.addFixableDiagnostic(
					{
						fixed: signals.replace({
							...node,
							attributes: newAttributes,
							children: [],
							selfClosing: true,
						}),
					},
					descriptions.LINT.REACT_NO_VOID_ELEMENTS_WITH_CHILDREN(
						element,
						Array.from(properties),
					),
				);
			}
		}

		return signals.retain;
	},
});
