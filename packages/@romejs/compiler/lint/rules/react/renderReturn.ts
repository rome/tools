import {Path, TransformExitResult} from "@romejs/compiler";
import {descriptions} from "@romejs/diagnostics";
import {AnyNode} from "@romejs/ast";
import {getCompletionRecords} from "@romejs/js-ast-utils";

function isRenderMethodOrProperty(node: AnyNode) {
	return (
		(node.type === "JSClassMethod" || node.type === "JSClassProperty") &&
		node.key.type === "JSStaticPropertyKey" &&
		node.key.value.type === "JSIdentifier" &&
		node.key.value.name === "render"
	);
}

function getMethodBody(node: AnyNode) {
	if (node.type === "JSClassMethod") {
		return node.body;
	}

	if (
		node.type === "JSClassProperty" &&
		node.value?.type === "JSArrowFunctionExpression"
	) {
		return node.value.body;
	}

	return undefined;
}

export default {
	name: "renderReturn",

	enter(path: Path): TransformExitResult {
		const {node, scope} = path;
		const reactIsInScope = scope.getBinding("React") !== undefined;

		if (reactIsInScope && isRenderMethodOrProperty(node)) {
			const body = getMethodBody(node);
			if (
				body &&
				!getCompletionRecords(body).some(({type}) => type === "COMPLETION")
			) {
				path.context.addNodeDiagnostic(
					node,
					descriptions.LINT.REACT_RENDER_RETURN,
				);
			}
		}
		return node;
	},
};
