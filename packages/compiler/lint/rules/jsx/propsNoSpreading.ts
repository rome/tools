import {Path, TransformExitResult} from "@romefrontend/compiler";
import {descriptions} from "@romefrontend/diagnostics";

export default {
	name: "jsx/propsNoSpreading",
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
