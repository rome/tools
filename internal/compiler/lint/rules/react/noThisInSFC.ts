import {createLintVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";
import {isFunctionNode} from "@internal/js-ast-utils";

export default createLintVisitor({
	name: "react/noThisInSFC",
	enter(path) {
		const {node} = path;

		if (node.type === "JSThisExpression") {
			const hasJSX = path.findAncestry((path) => {
				if (path.node.type === "JSXElement") {
					return true;
				}

				if (path.node.type === "JSBlockStatement") {
					if (
						path.node.body.some((statement) =>
							statement.type === "JSReturnStatement" &&
							statement.argument &&
							statement.argument.type === "JSXElement"
						)
					) {
						return true;
					}
				}

				const {node} = path;
				return (
					node.type === "JSVariableDeclarator" &&
					node.init !== undefined &&
					node.init.type === "JSArrowFunctionExpression" &&
					node.init.body.type === "JSXElement"
				);
			});

			if (hasJSX) {
				const isFunction = hasJSX.findAncestry((path) => {
					return isFunctionNode(path.node);
				});

				const declaration = hasJSX.node.type === "JSVariableDeclarator";

				if (isFunction || declaration) {
					path.context.addNodeDiagnostic(
						node,
						descriptions.LINT.REACT_NO_THIS_IN_SFC,
					);
				}
			}
		}

		return signals.retain;
	},
});
