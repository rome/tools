import {createVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";

export default createVisitor({
	name: "jsx/noPropSpreading",
	enter(path) {
		const {node} = path;

		if (node.type === "JSXSpreadAttribute") {
			path.context.addNodeDiagnostic(
				node,
				descriptions.LINT.JSX_NO_PROP_SPREADING,
			);
		}

		return signals.retain;
	},
});
