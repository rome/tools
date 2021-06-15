import {descriptions} from "@internal/diagnostics";
import {
	getJSXAttribute,
	hasJSXAttribute,
	isEmptyTemplateLiteral,
	isJSXElement,
} from "@internal/js-ast-utils";
import {JSXElement} from "@internal/ast";
import {createLintVisitor, signals} from "@internal/compiler";
import {isJSXDOMElement} from "@internal/js-ast-utils/isJSXDOMElement";
import getHTMLAttribute from "@internal/js-ast-utils/getHTMLAttribute";
import isHTMLElement from "@internal/js-ast-utils/isHTMLElement";

function validTitle(node: JSXElement) {
	if (isJSXDOMElement(node) && hasJSXAttribute(node, "title")) {
		const attr = getJSXAttribute(node, "title");
		if (attr?.value) {
			if (attr.value.type === "JSXExpressionContainer") {
				const expression = attr.value.expression;
				if (expression.type === "JSTemplateLiteral") {
					return !isEmptyTemplateLiteral(expression);
				}
				return (
					expression.type !== "JSNumericLiteral" &&
					expression.type !== "JSBooleanLiteral"
				);
			} else if (attr.value.type === "JSStringLiteral") {
				return attr.value.value !== "";
			}
			return false;
		}
	}
	return false;
}

export default createLintVisitor({
	name: "a11y/useIframeTitle",

	enter(path) {
		const {node} = path;
		if (isJSXElement(node, "iframe")) {
			if (!(hasJSXAttribute(node, "title") && validTitle(node))) {
				path.context.addNodeDiagnostic(
					node,
					descriptions.LINT.A11Y_IFRAME_USE_TITLE,
				);
			}
		} else if (isHTMLElement(node) && node.name.name === "iframe") {
			const titleAttribute = getHTMLAttribute(node, "title");
			if (!titleAttribute || titleAttribute.value?.value === "") {
				path.context.addNodeDiagnostic(
					node,
					descriptions.LINT.A11Y_IFRAME_USE_TITLE,
				);
			}
		}

		return signals.retain;
	},
});
