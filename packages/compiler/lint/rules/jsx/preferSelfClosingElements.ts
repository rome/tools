import {createVisitor, signals} from "@romefrontend/compiler";
import {descriptions} from "@romefrontend/diagnostics";

export default createVisitor({
	name: "jsx/preferSelfClosingElements",
	enter(path) {
		const {node} = path;

		if (
			node.type === "JSXElement" &&
			!node.selfClosing &&
			node.children.length === 0
		) {
			return path.addFixableDiagnostic(
				{
					fixed: signals.replace({
						...node,
						selfClosing: true,
					}),
				},
				descriptions.LINT.JSX_PREFER_SELF_CLOSING_ELEMENTS,
			);
		}

		return signals.retain;
	},
});
