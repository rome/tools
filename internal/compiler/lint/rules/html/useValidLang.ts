import {createVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";
import {getLangSuggestions} from "@internal/compiler/lint/utils/getLangSuggestions";
import {HTMLAttribute} from "@internal/ast";
import {langSupported} from "@internal/compiler/lint/utils/getLangSupported";


// Will return the attribute value if invalid
function isSupportedLang(attribute: HTMLAttribute): undefined | string {

		if (!langSupported(attribute.value.value)) {
			return attribute.value.value;
		}

	return undefined;
}

export default createVisitor({
	name: "html/useValidLang",
	enter(path) {
		const {node} = path;

		if (node.type === "HTMLElement" && node.name.name === "html") {
			const langAttr = node.attributes.find(a => a.name.name === "lang")
			if (langAttr !== undefined) {
				const invalidValue = isSupportedLang(langAttr)

				// TODO add an autofix suggestion
				if (invalidValue !== undefined) {
					path.context.addNodeDiagnostic(
						langAttr.value,
						descriptions.LINT.HTML_USE_VALID_LANG(invalidValue, getLangSuggestions()),
					);
				}

			}
		}

		return signals.retain;
	},
});
