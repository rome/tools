import {createLintVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";
import {HTMLAttribute, JSXElement} from "@internal/ast";
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

	if (!attr?.value) {
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

// Will return the attribute value if invalid
function htmlSupportedLang(attribute: HTMLAttribute): undefined | string {
	if (!langSupported(attribute.value?.value ?? "")) {
		return attribute.value?.value;
	}

	return undefined;
}

export default createLintVisitor({
	name: "a11y/useValidLang",
	enter(path) {
		const {node} = path;

		if (isJSXElement(node, "html") && hasJSXAttribute(node, "lang")) {
			const invalidValue = jsxSupportedLang(node);
			if (invalidValue !== undefined) {
				// TODO add an autofix suggestion
				path.context.addNodeDiagnostic(
					getJSXAttribute(node, "lang"),
					descriptions.LINT.A11Y_LANG(invalidValue, getLangSuggestions()),
				);
			}
		} else if (node.type === "HTMLElement" && node.name.name === "html") {
			const langAttr = node.attributes.find((a) => a.name.name === "lang");
			if (langAttr !== undefined) {
				const invalidValue = htmlSupportedLang(langAttr);

				if (invalidValue !== undefined) {
					path.context.addNodeDiagnostic(
						langAttr.value,
						descriptions.LINT.A11Y_LANG(invalidValue, getLangSuggestions()),
					);
				}
			}
		}

		return signals.retain;
	},
});
