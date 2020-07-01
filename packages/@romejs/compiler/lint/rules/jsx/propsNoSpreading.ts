import {Path, TransformExitResult} from "@romejs/compiler";
import {descriptions} from "@romejs/diagnostics";

export default {
	name: "jsxPropsNoSpreading",
	enter(path: Path): TransformExitResult {
		const {node} = path;

		if (node.type === "JSXSpreadAttribute") {
			path.context.addNodeDiagnostic(
				node,
				descriptions.LINT.JSX_PROPS_NO_SPREADING,
			);
		}

		return node;
	},
};
