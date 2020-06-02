import {descriptions} from "@romejs/diagnostics";
import {AnyNode, JSXElement} from "@romejs/ast";
import {Path} from "@romejs/compiler";
import {
	getJSXAttribute,
	hasJSXAttribute,
	isJSXElement,
} from "@romejs/js-ast-utils";

function validLang(node: JSXElement) {
	if (hasJSXAttribute(node, "lang")) {
		const attr = getJSXAttribute(node, "lang");
		if (attr && attr.value) {
			if (attr.value.type === "JSXExpressionContainer") {
				const expression = attr.value.expression;
				console.log(expression);
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
