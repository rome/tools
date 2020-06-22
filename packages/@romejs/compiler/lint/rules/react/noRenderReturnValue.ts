import {descriptions} from "@romejs/diagnostics";
import {AnyNode} from "@romejs/ast";
import {Path} from "@romejs/compiler";
import {doesNodeMatchReactPattern} from "../../utils/react";

export default {
	name: "reactNoRenderReturnValue",

	enter(path: Path): AnyNode {
		const {node, parent, scope} = path;

		if (
			node.type === "JSCallExpression" &&
			doesNodeMatchReactPattern(
				node.callee,
				scope,
				"ReactDOM.render",
				{
					packageName: "react-dom",
					importName: "ReactDOM",
				},
			) &&
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
