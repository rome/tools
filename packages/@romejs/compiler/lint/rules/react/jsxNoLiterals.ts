import {Path, TransformExitResult} from "@romejs/compiler";
import {AnyNode, JSStringLiteral, JSXText} from "@romejs/ast";
import {descriptions} from "@romejs/diagnostics";

function getUnWrappedLiteral(
	node: AnyNode,
): JSStringLiteral | JSXText | undefined {
	if (node.type === "JSXAttribute" && node.value?.type === "JSStringLiteral") {
		return node.value;
	}
	if (node.type === "JSXText" && node.value.trim().length > 0) {
		return node;
	}

	return undefined;
}

export default {
	name: "reactNoJsxLiterals",
	enter(path: Path): TransformExitResult {
		const {node, context} = path;

		const upWrappedLiteral = getUnWrappedLiteral(node);

		if (upWrappedLiteral !== undefined) {
			context.addNodeDiagnostic(
				upWrappedLiteral,
				descriptions.LINT.REACT_JSX_NO_LITERALS,
			);
		}
		return node;
	},
};
