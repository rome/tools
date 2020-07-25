import {Path, TransformExitResult} from "@romefrontend/compiler";
import {descriptions} from "@romefrontend/diagnostics";

export default {
	name: "jsx/preferSelfClosingElements",
	enter(path: Path): TransformExitResult {
		const {node} = path;

		if (
			node.type === "JSXElement" &&
			!node.selfClosing &&
			node.children.length === 0
		) {
			return path.context.addFixableDiagnostic(
				{
					old: node,
					fixed: {
						...node,
						selfClosing: true,
					},
				},
				descriptions.LINT.JSX_PREFER_SELF_CLOSING_ELEMENTS,
			);
		}

		return node;
	},
};
