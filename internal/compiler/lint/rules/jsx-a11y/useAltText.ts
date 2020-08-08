/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {createVisitor, signals} from "@internal/compiler";
import {JSXElement} from "@internal/ast";
import {descriptions} from "@internal/diagnostics";
import {
	getJSXAttribute,
	hasJSXAttribute,
	isJSXElement,
} from "@internal/js-ast-utils";

function hasImgAltText(node: JSXElement): boolean {
	const attr = getJSXAttribute(node, "alt", true);
	if (attr === undefined) {
		return false;
	}
	return (
		(attr.value &&
		attr.value.type === "JSStringLiteral" &&
		attr.value.value === "") ||
		hasJSXAttribute(node, "alt")
	);
}

function hasObjectAltText(node: JSXElement): boolean {
	return (
		hasJSXAttribute(node, "aria-label") ||
		hasJSXAttribute(node, "aria-labelledby") ||
		hasJSXAttribute(node, "title") ||
		node.children.length > 0
	);
}

function hasAreaAltText(node: JSXElement): boolean {
	return (
		hasJSXAttribute(node, "aria-label") ||
		hasJSXAttribute(node, "aria-labelledby") ||
		hasJSXAttribute(node, "alt") ||
		hasJSXAttribute(node, "title")
	);
}

function hasInputAltText(node: JSXElement): boolean {
	return (
		hasJSXAttribute(node, "aria-label") ||
		hasJSXAttribute(node, "aria-labelledby") ||
		hasJSXAttribute(node, "alt") ||
		hasJSXAttribute(node, "title")
	);
}

function hasTypeImage(node: JSXElement): boolean {
	const attr = getJSXAttribute(node, "type");
	if (attr === undefined) {
		return false;
	}
	return (
		attr.value !== undefined &&
		attr.value.type === "JSStringLiteral" &&
		attr.value.value === "image"
	);
}

export default createVisitor({
	name: "jsx-a11y/useAltText",
	enter(path) {
		const {node} = path;

		if (node.type === "JSXElement" && node.name.type === "JSXIdentifier") {
			if (!/(img)|(area)|(input)|(object)/.test(node.name.name)) {
				return signals.retain;
			}

			if (
				(isJSXElement(node, "img") && !hasImgAltText(node)) ||
				(isJSXElement(node, "object") && !hasObjectAltText(node)) ||
				(isJSXElement(node, "area") && !hasAreaAltText(node)) ||
				(isJSXElement(node, "input") &&
				hasTypeImage(node) &&
				!hasInputAltText(node))
			) {
				path.context.addNodeDiagnostic(
					node,
					descriptions.LINT.JSX_A11Y_ALT_TEXT,
				);
			}
		}
		return signals.retain;
	},
});
