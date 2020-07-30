import {createVisitor, signals} from "@romefrontend/compiler";
import {descriptions} from "@romefrontend/diagnostics";

export default createVisitor({
	name: "jsx/propsNoSpreading",
	enter(path) {
		const {node} = path;

		if (node.type === "JSXSpreadAttribute") {
			path.context.addNodeDiagnostic(
				node,
				descriptions.LINT.JSX_PROPS_NO_SPREADING,
			);
		}

		return signals.retain;
	},
});
