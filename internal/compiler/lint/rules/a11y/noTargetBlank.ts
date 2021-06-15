import {descriptions} from "@internal/diagnostics";
import {AnyNode, HTMLElement} from "@internal/ast";
import {createLintVisitor, signals} from "@internal/compiler";
import {getJSXAttribute, isJSXElement} from "@internal/js-ast-utils";
import {isJSXDOMElement} from "@internal/js-ast-utils/isJSXDOMElement";
import isHTMLElement from "@internal/js-ast-utils/isHTMLElement";

import getHTMLAttribute from "@internal/js-ast-utils/getHTMLAttribute";

const EXTERNAL_LINK_REGEX = /^(?:\w+:|\/\/)/;

const TARGET = "target";
const BLANK = "_blank";
const REL = "rel";
const NOREFERRER = "noreferrer";
const HREF = "href";

function jsxAnchorHasBlankTarget(node: AnyNode) {
	return (
		isJSXElement(node, "a") &&
		node.attributes.some((attribute) =>
			attribute.type === "JSXAttribute" &&
			attribute.name.name === TARGET &&
			attribute.value &&
			attribute.value.type === "JSStringLiteral" &&
			attribute.value.value === BLANK
		)
	);
}

function jsxAnchorHasNoReferrer(node: AnyNode) {
	return (
		isJSXElement(node, "a") &&
		node.attributes.some((attribute) =>
			attribute.type === "JSXAttribute" &&
			attribute.name.name === REL &&
			attribute.value &&
			attribute.value.type === "JSStringLiteral" &&
			attribute.value.value.includes(NOREFERRER)
		)
	);
}

function jsxAnchorHasExternalLink(node: AnyNode) {
	return (
		isJSXElement(node, "a") &&
		node.attributes.some((attribute) =>
			attribute.type === "JSXAttribute" &&
			attribute.name.name === HREF &&
			attribute.value &&
			((attribute.value.type === "JSStringLiteral" &&
			EXTERNAL_LINK_REGEX.test(attribute.value.value)) ||
			attribute.value.type === "JSXExpressionContainer")
		)
	);
}

function anchorHasBlankTarget(node: HTMLElement) {
	return node.attributes?.some((a) =>
		a.name.name === TARGET && a.value && a.value.value === BLANK
	);
}

function anchorHasNoReferrer(node: HTMLElement) {
	return node.attributes?.some((a) =>
		a.name.name === REL && a.value && a.value.value.includes(NOREFERRER)
	);
}

function anchorHasExternalLink(node: HTMLElement) {
	return node.attributes?.some((a) =>
		a.name.name === HREF && a.value && EXTERNAL_LINK_REGEX.test(a.value.value)
	);
}

export default createLintVisitor({
	name: "a11y/noTargetBlank",

	enter(path) {
		const {node} = path;

		if (isHTMLElement(node) && node.name.name === "a") {
			if (
				anchorHasBlankTarget(node) &&
				!anchorHasNoReferrer(node) &&
				anchorHasExternalLink(node)
			) {
				return path.addFixableDiagnostic(
					{
						target: getHTMLAttribute(node, "target"),
						fixed: signals.replace({
							...node,
							attributes: node.attributes.filter((attribute) =>
								attribute.name.name !== "target"
							),
						}),
					},
					descriptions.LINT.A11_Y_NO_TARGET_BLANK,
				);
			}
		} else if (
			isJSXDOMElement(node) &&
			jsxAnchorHasBlankTarget(node) &&
			!jsxAnchorHasNoReferrer(node) &&
			jsxAnchorHasExternalLink(node)
		) {
			return path.addFixableDiagnostic(
				{
					target: getJSXAttribute(node, "target"),
					fixed: signals.replace({
						...node,
						attributes: node.attributes.filter((attribute) =>
							attribute.type !== "JSXAttribute" ||
							attribute.name.name !== "target"
						),
					}),
				},
				descriptions.LINT.A11_Y_NO_TARGET_BLANK,
			);
		}

		return signals.retain;
	},
});
