import {createVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";
import {JSXElement} from "@internal/ast";
import {
	getJSXAttribute,
	hasJSXAttribute,
	isJSXElement,
} from "@internal/js-ast-utils";
import {getLangSuggestions} from "@internal/compiler/lint/utils/getLangSuggestions";
import {langSupported} from "@internal/compiler/lint/utils/getLangSupported";

// Will return the attribute value if invalid
function jsxSupportedLang(node: JSXElement): undefined | string {
	const attr = getJSXAttribute(node, "lang");

	if (!attr || !attr.value) {
		return "undefined";
	}

	if (attr.value.type === "JSStringLiteral") {
		const {value} = attr.value;
		if (!langSupported(value)) {
			return value;
		}
	}

	return undefined;
}

export default createVisitor({
	name: "jsx-a11y/useValidLang",
	enter(path) {
		const {node} = path;

		if (isJSXElement(node, "html") && hasJSXAttribute(node, "lang")) {
			const invalidValue = jsxSupportedLang(node);
			if (invalidValue !== undefined) {
				// TODO add an autofix suggestion
				path.context.addNodeDiagnostic(
					getJSXAttribute(node, "lang"),
					descriptions.LINT.JSX_A11Y_LANG(invalidValue, getLangSuggestions()),
				);
			}
		}

		return signals.retain;
	},
});
