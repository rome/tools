import {createVisitor, signals} from "@romefrontend/compiler";
import {descriptions} from "@romefrontend/diagnostics";
import {VOID_DOM_ELEMENTS} from "../../utils/constants";

export default createVisitor({
	name: "html/preferClosingNonVoid",
	enter(path) {
		const {node} = path;

		if (
			node.type === "HTMLElement" &&
			node.selfClosing &&
			!VOID_DOM_ELEMENTS.has(node.name.name)
		) {
			return path.addFixableDiagnostic(
				{
					fixed: signals.replace({
						...node,
						selfClosing: false,
					}),
				},
				descriptions.LINT.HTML_PREFER_CLOSING_NON_VOID,
			);
		}

		return signals.retain;
	},
});
