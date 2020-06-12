import {descriptions} from "@romejs/diagnostics";
import {AnyNode} from "@romejs/ast";
import {Path} from "@romejs/compiler";
import {doesNodeMatchPattern} from "@romejs/js-ast-utils";

function hasReactDomRender(node: AnyNode | undefined): boolean {
	if (node) {
		if (node.type === "JSLogicalExpression") {
			return hasReactDomRender(node.left) || hasReactDomRender(node.right);
		} else if (node.type === "JSConditionalExpression") {
			return (
				hasReactDomRender(node.consequent) || hasReactDomRender(node.alternate)
			);
		}
		return (
			node.type === "JSCallExpression" &&
			doesNodeMatchPattern(node.callee, "ReactDOM.render")
		);
	}

	return false;
}

function getNodeToCheck(node: AnyNode): AnyNode | undefined {
	switch (node.type) {
		case "JSVariableDeclarator":
			return node.init;
		case "JSObjectProperty":
			return node.value;
		case "JSReturnStatement":
			return node.argument;
		case "JSAssignmentExpression":
			return node.right;
		case "JSArrowFunctionExpression":
			return node.body;
	}

	return undefined;
}

export default {
	name: "reactNoRenderReturnValue",

	enter(path: Path): AnyNode {
		const {node} = path;

		const nodeToCheck = getNodeToCheck(node);
		if (nodeToCheck && hasReactDomRender(nodeToCheck)) {
			path.context.addNodeDiagnostic(
				nodeToCheck,
				descriptions.LINT.REACT_NO_RENDER_RETURN_VALUE,
			);
		}

		return node;
	},
};
