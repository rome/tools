import {Path, TransformExitResult} from "@romejs/compiler";
import {descriptions} from "@romejs/diagnostics";
import {insideClassComponent} from "@romejs/compiler/lint/utils/react";

const ALLOWED_EXTENSIONS = [".jsx", ".tsx"];

export default {
	name: "reactFilenameExtension",
	enter(path: Path): TransformExitResult {
		const {node, context} = path;

		if (
			node.type === "JSClassHead" &&
			insideClassComponent(path) &&
			ALLOWED_EXTENSIONS.includes(context.path.getExtensions()) === false
		) {
			context.addNodeDiagnostic(
				node,
				descriptions.LINT.REACT_FILENAME_EXTENSION(context.path.getExtensions()),
			);
		}

		return node;
	},
};
