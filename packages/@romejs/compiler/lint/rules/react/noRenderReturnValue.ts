import {descriptions} from "@romejs/diagnostics";
import {AnyNode} from "@romejs/ast";
import {Path} from "@romejs/compiler";
import {doesNodeMatchPattern} from "@romejs/js-ast-utils";

export default {
	name: "reactNoRenderReturnValue",

	enter(path: Path): AnyNode {
		const {node, parent} = path;

		if (
			node.type === "JSCallExpression" &&
			doesNodeMatchPattern(node.callee, "ReactDOM.render") &&
			parent.type !== "JSExpressionStatement"
		) {
			path.context.addNodeDiagnostic(
				node,
				descriptions.LINT.REACT_NO_RENDER_RETURN_VALUE,
			);
		}

		return node;
	},
};
