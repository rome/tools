import {createVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";

export default createVisitor({
	name: "html/useHtmlLang",
	enter(path) {
		const {node} = path;

		if (node.type === "HTMLElement" && node.name.name === "html") {
			const langAttr = node.attributes.find((a) => a.name.name === "lang");
			if (!langAttr || langAttr?.value?.value === "") {
				path.context.addNodeDiagnostic(
					node,
					descriptions.LINT.HTML_USE_HTML_LANG,
				);
			}
		}

		return signals.retain;
	},
});
