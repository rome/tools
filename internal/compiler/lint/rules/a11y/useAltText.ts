import {createLintVisitor, signals} from "@internal/compiler";
import {HTMLElement, JSXElement} from "@internal/ast";
import {descriptions} from "@internal/diagnostics";
import {
	getJSXAttribute,
	hasJSXAttribute,
	isJSXElement,
} from "@internal/js-ast-utils";
import getHTMLAttribute from "@internal/js-ast-utils/getHTMLAttribute";
import isHTMLElement from "@internal/js-ast-utils/isHTMLElement";
import htmlAttributeHasValue from "@internal/js-ast-utils/htmlAttributeHasValue";

function hasImgAltText(node: JSXElement | HTMLElement): boolean {
	if (node.type === "JSXElement") {
		const attr = getJSXAttribute(node, "alt", true);
		if (attr === undefined) {
			return false;
		}
		return (
			(attr.value?.type === "JSStringLiteral" && attr.value.value === "") ||
			hasJSXAttribute(node, "alt")
		);
	}

	return htmlAttributeHasValue(node, "alt");
}

function hasObjectAltText(node: JSXElement | HTMLElement): boolean {
	if (node.type === "JSXElement") {
		return (
			hasJSXAttribute(node, "aria-label") ||
			hasJSXAttribute(node, "aria-labelledby") ||
			hasJSXAttribute(node, "title") ||
			node.children.length > 0
		);
	}
	return (
		htmlAttributeHasValue(node, "aria-label") ||
		htmlAttributeHasValue(node, "aria-labelledby") ||
		htmlAttributeHasValue(node, "title") ||
		node.children.length > 0
	);
}

function hasAreaAltText(node: JSXElement | HTMLElement): boolean {
	if (node.type === "JSXElement") {
		return (
			hasJSXAttribute(node, "aria-label") ||
			hasJSXAttribute(node, "aria-labelledby") ||
			hasJSXAttribute(node, "alt") ||
			hasJSXAttribute(node, "title")
		);
	}
	return (
		htmlAttributeHasValue(node, "aria-label") ||
		htmlAttributeHasValue(node, "aria-labelledby") ||
		htmlAttributeHasValue(node, "alt") ||
		htmlAttributeHasValue(node, "title")
	);
}

function hasInputAltText(node: JSXElement | HTMLElement): boolean {
	if (node.type === "JSXElement") {
		return (
			hasJSXAttribute(node, "aria-label") ||
			hasJSXAttribute(node, "aria-labelledby") ||
			hasJSXAttribute(node, "alt") ||
			hasJSXAttribute(node, "title")
		);
	}

	return (
		htmlAttributeHasValue(node, "aria-label") ||
		htmlAttributeHasValue(node, "aria-labelledby") ||
		htmlAttributeHasValue(node, "alt") ||
		htmlAttributeHasValue(node, "title")
	);
}

function hasTypeImage(node: JSXElement | HTMLElement): boolean {
	if (node.type === "JSXElement") {
		const attr = getJSXAttribute(node, "type");
		if (attr === undefined) {
			return false;
		}
		return (
			attr.value?.type === "JSStringLiteral" && attr.value.value === "image"
		);
	}
	const attr = getHTMLAttribute(node, "type");

	return attr?.value?.type === "HTMLString" && attr.value.value === "image";
}

function isImportantTag(nodeName: string) {
	return /(img)|(area)|(input)|(object)/.test(nodeName);
}

export default createLintVisitor({
	name: "a11y/useAltText",
	enter(path) {
		const {node} = path;

		if (isHTMLElement(node)) {
			if (!isImportantTag(node.name.name)) {
				return signals.retain;
			}

			if (
				(node.name.name === "img" && !hasImgAltText(node)) ||
				(node.name.name === "object" && !hasObjectAltText(node)) ||
				(node.name.name === "area" && !hasAreaAltText(node)) ||
				(node.name.name === "input" &&
				hasTypeImage(node) &&
				!hasInputAltText(node))
			) {
				path.context.addNodeDiagnostic(node, descriptions.LINT.A11Y_ALT_TEXT);
			}
		}

		if (node.type === "JSXElement" && node.name.type === "JSXIdentifier") {
			if (!isImportantTag(node.name.name)) {
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
				path.context.addNodeDiagnostic(node, descriptions.LINT.A11Y_ALT_TEXT);
			}
		}
		return signals.retain;
	},
});
