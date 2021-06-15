import {createLintVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";
import {
	getJSXAttribute,
	hasJSXAttribute,
	isJSXElement,
} from "@internal/js-ast-utils";
import {JSXAttribute} from "@internal/ast";
import {markup} from "@internal/markup";
import isHTMLElement from "@internal/js-ast-utils/isHTMLElement";
import getHTMLAttribute from "@internal/js-ast-utils/getHTMLAttribute";
import hasHTMLAttribute from "@internal/js-ast-utils/hasHTMLAttribute";

function hrefValue(attr: JSXAttribute | undefined, value: string): boolean {
	if (attr === undefined) {
		return false;
	}
	return (
		(attr?.value?.type === "JSStringLiteral" && attr.value.value === value) ||
		(attr?.value?.type === "JSXExpressionContainer" &&
		attr.value.expression.type === "JSStringLiteral" &&
		attr.value.expression.value === value) ||
		(attr?.value?.type === "JSXExpressionContainer" &&
		attr.value.expression.type === "JSTemplateLiteral" &&
		attr.value.expression.quasis.some((element) => element.raw === value))
	);
}

function falsyHref(attr: JSXAttribute | undefined): boolean {
	return (
		attr === undefined ||
		(attr.value?.type === "JSXExpressionContainer" &&
		attr.value.expression.type === "JSNullLiteral")
	);
}

export default createLintVisitor({
	name: "a11y/useValidAnchor",
	enter(path) {
		const {node} = path;

		if (isJSXElement(node, "a")) {
			const attr = getJSXAttribute(node, "href");

			if (falsyHref(attr) && !hasJSXAttribute(node, "onClick")) {
				path.context.addNodeDiagnostic(
					node,
					descriptions.LINT.A11Y_ANCHOR_IS_VALID(
						markup`Provide a <emphasis>href</emphasis> attribute for the <emphasis>a</emphasis> element.`,
					),
				);
			}

			if (attr && !hasJSXAttribute(node, "onClick")) {
				if (hrefValue(attr, "#") || hrefValue(attr, "javascript:void(0)")) {
					path.context.addNodeDiagnostic(
						node,
						descriptions.LINT.A11Y_ANCHOR_IS_VALID(
							markup`Provide a valid <emphasis>href</emphasis> attribute for the <emphasis>a</emphasis> element.`,
						),
					);
				}
			}

			if (
				(hasJSXAttribute(node, "onClick") && !hasJSXAttribute(node, "href")) ||
				(hasJSXAttribute(node, "href") && hrefValue(attr, "#")) ||
				hrefValue(attr, "javascript:void(0)")
			) {
				path.context.addNodeDiagnostic(
					node,
					descriptions.LINT.A11Y_ANCHOR_IS_VALID(
						markup`Use a <emphasis>button</emphasis> element instead of an <emphasis>a</emphasis> element.`,
					),
				);
			}
		} else if (isHTMLElement(node) && node.name.name === "a") {
			const attr = getHTMLAttribute(node, "href");

			if (attr === undefined && !hasHTMLAttribute(node, "onclick")) {
				path.context.addNodeDiagnostic(
					node,
					descriptions.LINT.A11Y_ANCHOR_IS_VALID(
						markup`Provide a <emphasis>href</emphasis> attribute for the <emphasis>a</emphasis> element.`,
					),
				);
			}

			if (attr && !hasHTMLAttribute(node, "onclick")) {
				if (
					attr.value?.value === "#" ||
					attr.value?.value === "javascript:void(0)"
				) {
					path.context.addNodeDiagnostic(
						node,
						descriptions.LINT.A11Y_ANCHOR_IS_VALID(
							markup`Provide a valid <emphasis>href</emphasis> attribute for the <emphasis>a</emphasis> element.`,
						),
					);
				}
			}

			if (
				(hasHTMLAttribute(node, "onclick") && !hasHTMLAttribute(node, "href")) ||
				(hasHTMLAttribute(node, "href") && attr?.value?.value === "#") ||
				(hasHTMLAttribute(node, "href") &&
				attr?.value?.value === "javascript:void(0)")
			) {
				path.context.addNodeDiagnostic(
					node,
					descriptions.LINT.A11Y_ANCHOR_IS_VALID(
						markup`Use a <emphasis>button</emphasis> element instead of an <emphasis>a</emphasis> element.`,
					),
				);
			}
		}

		return signals.retain;
	},
});
