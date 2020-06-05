import {Path, TransformExitResult} from "@romejs/compiler";
import {descriptions} from "@romejs/diagnostics";
import {AnyJSClassMember, AnyNode, JSClassHead} from "@romejs/ast";
import {doesNodeMatchPattern, getCompletionRecords} from "@romejs/js-ast-utils";

function isRenderProperty(node: AnyNode) {
	return (
		node.type === "JSStaticPropertyKey" &&
		node.value.type === "JSIdentifier" &&
		node.value.name === "render"
	);
}

function getMethodBody(node: AnyJSClassMember) {
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

function isExtendingReactComponent(node: JSClassHead) {
	return (
		doesNodeMatchPattern(node.superClass, "React.Component") ||
		doesNodeMatchPattern(node.superClass, "Component")
	);
}

export default {
	name: "requireRenderReturn",
	enter(path: Path): TransformExitResult {
		const {node} = path;
		if (node.type === "JSClassHead" && isExtendingReactComponent(node)) {
			const renderMember = node.body.find(({key}) => isRenderProperty(key));
			const renderBody = renderMember && getMethodBody(renderMember);

			if (
				renderBody &&
				!getCompletionRecords(renderBody).some(({type}) => type === "COMPLETION")
			) {
				path.context.addNodeDiagnostic(
					renderBody,
					descriptions.LINT.REACT_REQUIRE_RENDER_RETURN,
				);
			}
		}
		return node;
	},
};
