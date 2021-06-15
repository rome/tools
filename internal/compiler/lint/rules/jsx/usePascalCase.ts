import {createLintVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";
import {normalizeCamelCase} from "../js/useCamelCase";
import {toCamelCase} from "@internal/string-utils";

export default createLintVisitor({
	name: "jsx/usePascalCase",
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
					descriptions.LINT.JSX_USE_PASCAL_CASE(node.name.name, pascalCaseName),
				);
			}
		}
		return signals.retain;
	},
});
