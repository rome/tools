import {Path, TransformExitResult} from "@romefrontend/compiler";
import {descriptions} from "@romefrontend/diagnostics";
import {VOID_DOM_ELEMENTS} from "../../utils/constants";

export default {
	name: "html/preferClosingNonVoid",
	enter(path: Path): TransformExitResult {
		const {node} = path;

		if (
			node.type === "HTMLElement" &&
			node.selfClosing &&
			!VOID_DOM_ELEMENTS.has(node.name.name)
		) {
			return path.context.addFixableDiagnostic(
				{
					old: node,
					fixed: {
						...node,
						selfClosing: false,
					},
				},
				descriptions.LINT.HTML_PREFER_CLOSING_NON_VOID,
			);
		}

		return node;
	},
};
