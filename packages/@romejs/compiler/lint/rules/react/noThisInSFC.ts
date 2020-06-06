import {Path, TransformExitResult} from "@romejs/compiler";
import {descriptions} from "@romejs/diagnostics";
import {isFunctionNode, isJSXElement} from "@romejs/js-ast-utils";

export default {
	name: "reactNoThisInSFC",
	enter(path: Path): TransformExitResult {
		const {node} = path;

		if (node.type === "JSThisExpression") {
			const hasJSX = path.findAncestry((path) => {
				if (isJSXElement(path.node)) {
					return true;
				}

				if (path.node.type === "JSBlockStatement") {
					if (
						path.node.body.some((statement) =>
							statement.type === "JSReturnStatement" &&
							statement.argument &&
							isJSXElement(statement.argument)
						)
					) {
						return true;
					}
				}

				if (
					path.node.type === "JSVariableDeclarator" &&
					path.node.init &&
					path.node.init.type === "JSArrowFunctionExpression" &&
					isJSXElement(path.node.init.body)
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
