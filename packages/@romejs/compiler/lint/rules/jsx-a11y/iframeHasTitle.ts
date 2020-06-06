import {descriptions} from "@romejs/diagnostics";
import {
	getJSXAttribute,
	hasJSXAttribute,
	isJSXElement,
} from "@romejs/js-ast-utils";
import {AnyNode, JSXElement} from "@romejs/ast";
import {Path} from "@romejs/compiler";
import isEmptyTemplateLiteral from "@romejs/js-ast-utils/isEmptyTemplateLiteral";

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
				if (!attr.value.value) {
					return false;
				}
				return true;
			}
			return false;
		}
	}
	return false;
}

export default {
	name: "jsxA11YIframeHasTitle",

	enter(path: Path): AnyNode {
		const {node} = path;
		if (isJSXElement(node, "iframe")) {
			if (!hasJSXAttribute(node, "title") || !validTitle(node)) {
				path.context.addNodeDiagnostic(
					node,
					descriptions.LINT.JSX_A11Y_IFRAME_HAS_TITLE,
				);
			}
		}

		return node;
	},
};
