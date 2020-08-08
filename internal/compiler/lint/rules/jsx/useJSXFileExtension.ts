import {CompilerContext, createVisitor, signals} from "@internal/compiler";

import {descriptions} from "@internal/diagnostics";
import {AnyNode} from "@internal/ast";

const JSX_USE_J_S_X_FILE_EXTENSIONS = [".jsx", ".tsx"];

function isJSXNode(node: AnyNode): boolean {
	return node.type === "JSXFragment" || node.type === "JSXElement";
}

function isJSXFile(context: CompilerContext): boolean {
	return JSX_USE_J_S_X_FILE_EXTENSIONS.includes(context.path.getExtensions());
}

export default createVisitor({
	name: "jsx/useJSXFileExtension",
	enter(path) {
		const {node, context} = path;

		if (isJSXNode(node) && !isJSXFile(context)) {
			path.context.addNodeDiagnostic(
				node,
				descriptions.LINT.JSX_USE_J_S_X_FILE_EXTENSION(
					context.path.getExtensions(),
					context.path.getExtensionlessBasename(),
				),
			);
		}

		return signals.retain;
	},
});
