import {Path, TransformExitResult} from "@romefrontend/compiler";
import {descriptions} from "@romefrontend/diagnostics";
import {toCamelCase} from "@romefrontend/string-utils";

export default {
	name: "jsxPascalCase",
	enter(path: Path): TransformExitResult {
		const {node} = path;

		if (
			node.type === "JSXElement" &&
			node.name.type === "JSXReferenceIdentifier"
		) {
			const pascalCaseName = toCamelCase(node.name.name, true);
			if (node.name.name !== pascalCaseName) {
				path.context.addNodeDiagnostic(
					node,
					descriptions.LINT.JSX_PASCAL_CASE(node.name.name, pascalCaseName),
				);
			}
		}
		return node;
	},
};
