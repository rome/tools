import {descriptions} from "@romefrontend/diagnostics";
import {AnyNode, JSXElement} from "@romefrontend/ast";
import {Path} from "@romefrontend/compiler";
import {
	getJSXAttribute,
	hasJSXAttribute,
	isEmptyTemplateLiteral,
	isJSXElement,
} from "@romefrontend/js-ast-utils";

function validLang(node: JSXElement) {
	if (hasJSXAttribute(node, "lang")) {
		const attr = getJSXAttribute(node, "lang");
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
				return true;
			}
			return false;
		}
	}
	return false;
}

export default {
	name: "jsxA11YHTMLHasLang",

	enter(path: Path): AnyNode {
		const {node} = path;
		if (isJSXElement(node, "html")) {
			if (!hasJSXAttribute(node, "lang") || !validLang(node)) {
				path.context.addNodeDiagnostic(
					node,
					descriptions.LINT.JSX_A11Y_HTML_HAS_LANG,
				);
			}
		}

		return node;
	},
};
