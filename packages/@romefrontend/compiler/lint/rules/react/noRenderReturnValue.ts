import {descriptions} from "@romefrontend/diagnostics";
import {AnyNode} from "@romefrontend/ast";
import {Path} from "@romefrontend/compiler";
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
