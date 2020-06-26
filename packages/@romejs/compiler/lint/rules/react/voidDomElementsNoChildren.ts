/**
* Copyright (c) Facebook, Inc. and its affiliates.
*
* This source code is licensed under the MIT license found in the
* LICENSE file in the root directory of this source tree.
*/

import {Path, REDUCE_REMOVE} from "@romejs/compiler";

import {descriptions} from "@romejs/diagnostics";
import {TransformExitResult} from "@romejs/compiler/types";
import {JSStringLiteral} from "@romejs/ast";
import {
	getCreateElementChildren,
	getCreateElementProp,
	getCreateElementType,
	isCreateElement,
} from "../../utils/react";

const VOID_DOM_ELEMENTS = new Set([
	"area",
	"base",
	"br",
	"col",
	"embed",
	"hr",
	"img",
	"input",
	"keygen",
	"link",
	"menuitem",
	"meta",
	"param",
	"source",
	"track",
	"wbr",
]);

export default {
	name: "reactVoidDomElementsNoChildren",
	enter(path: Path): TransformExitResult {
		const {node, context, scope} = path;
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
				context.addFixableDiagnostic(
					{
						target: node.arguments,
						old: node,
						fixed: {
							...node,
							arguments: [node.arguments[0], node.arguments[1]],
						},
					},
					descriptions.LINT.REACT_VOID_DOM_ELEMENTS_NO_CHILDREN(
						(node.arguments[0] as JSStringLiteral).value,
						["children"],
					),
				);
			} else if (elementType && childrenNode) {
				context.addFixableDiagnostic(
					{
						old: childrenNode,
						fixed: REDUCE_REMOVE,
					},
					descriptions.LINT.REACT_VOID_DOM_ELEMENTS_NO_CHILDREN(
						elementType,
						["children"],
					),
				);
			}

			if (elementType && dangerNode) {
				context.addFixableDiagnostic(
					{
						old: dangerNode,
						fixed: REDUCE_REMOVE,
					},
					descriptions.LINT.REACT_VOID_DOM_ELEMENTS_NO_CHILDREN(
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
					} else if (property === "dangerouslySetInnerHTML") {
						properties.add("dangerouslySetInnerHTML");
					} else {
						newAttributes.push(attribute);
					}
				}
			}

			if (properties.size > 0) {
				return context.addFixableDiagnostic(
					{
						old: node,
						fixed: {
							...node,
							attributes: newAttributes,
							children: [],
							selfClosing: true,
						},
					},
					descriptions.LINT.REACT_VOID_DOM_ELEMENTS_NO_CHILDREN(
						element,
						Array.from(properties),
					),
				);
			}
		}

		return node;
	},
};
