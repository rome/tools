import {createVisitor, signals} from "@romefrontend/compiler";
import {descriptions} from "@romefrontend/diagnostics";
import {AnyJSClassMember, AnyNode} from "@romefrontend/ast";
import {getCompletionRecords} from "@romefrontend/js-ast-utils";
import {insideClassComponent} from "../../utils/react";

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

export default createVisitor({
	name: "requireRenderReturn",
	enter(path) {
		const {node} = path;
		if (node.type === "JSClassHead" && insideClassComponent(path)) {
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
		return signals.retain;
	},
});
