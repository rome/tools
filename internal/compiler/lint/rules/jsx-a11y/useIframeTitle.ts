import {descriptions} from "@internal/diagnostics";
import {
	getJSXAttribute,
	hasJSXAttribute,
	isEmptyTemplateLiteral,
	isJSXElement,
} from "@internal/js-ast-utils";
import {JSXElement} from "@internal/ast";
import {createVisitor, signals} from "@internal/compiler";

function validTitle(node: JSXElement) {
	if (hasJSXAttribute(node, "title")) {
		const attr = getJSXAttribute(node, "title");
		if (attr && attr.value) {
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

export default createVisitor({
	name: "jsx-a11y/useIframeTitle",

	enter(path) {
		const {node} = path;
		if (isJSXElement(node, "iframe")) {
			if (!hasJSXAttribute(node, "title") || !validTitle(node)) {
				path.context.addNodeDiagnostic(
					node,
					descriptions.LINT.JSX_A11Y_IFRAME_HAS_TITLE,
				);
			}
		}

		return signals.retain;
	},
});
