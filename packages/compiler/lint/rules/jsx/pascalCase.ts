import {createVisitor, signals} from "@romefrontend/compiler";
import {descriptions} from "@romefrontend/diagnostics";
import {normalizeCamelCase} from "../js/camelCase";
import {toCamelCase} from "@romefrontend/string-utils";

export default createVisitor({
	name: "jsx/pascalCase",
	enter(path) {
		const {node} = path;

		if (
			node.type === "JSXElement" &&
			node.name.type === "JSXReferenceIdentifier"
		) {
			const pascalCaseName = normalizeCamelCase(
				toCamelCase(
					node.name.name,
					{
						allowShouty: false,
						forcePascal: true,
					},
				),
			);
			if (pascalCaseName !== undefined && node.name.name !== pascalCaseName) {
				path.context.addNodeDiagnostic(
					node,
					descriptions.LINT.JSX_PASCAL_CASE(node.name.name, pascalCaseName),
				);
			}
		}
		return signals.retain;
	},
});
