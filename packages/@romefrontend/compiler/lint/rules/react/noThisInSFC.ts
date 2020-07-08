import {Path, TransformExitResult} from "@romefrontend/compiler";
import {descriptions} from "@romefrontend/diagnostics";
import {isFunctionNode} from "@romefrontend/js-ast-utils";

export default {
	name: "reactNoThisInSFC",
	enter(path: Path): TransformExitResult {
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

				if (
					path.node.type === "JSVariableDeclarator" &&
					path.node.init &&
					path.node.init.type === "JSArrowFunctionExpression" &&
					path.node.init.body.type === "JSXElement"
				) {
					return true;
				}
				return false;
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

		return node;
	},
};
