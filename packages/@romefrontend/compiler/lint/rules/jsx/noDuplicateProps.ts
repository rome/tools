import {Path, TransformExitResult} from "@romefrontend/compiler";
import {descriptions} from "@romefrontend/diagnostics";
import {DiagnosticsDuplicateHelper} from "@romefrontend/compiler/lib/DiagnosticsDuplicateHelper";
import {JSXAttribute} from "@romefrontend/ast";

function getAttributeKey(node: JSXAttribute): string {
	const name = node.name.name;
	return typeof name === "string" ? name : name.name;
}

export default {
	name: "jsxNoDuplicateProps",

	enter(path: Path): TransformExitResult {
		const {context, node} = path;

		if (node.type !== "JSXElement") {
			return node;
		}

		const duplicates = new DiagnosticsDuplicateHelper(
			context,
			descriptions.LINT.JSX_NO_DUPLICATE_PROPS,
		);

		for (const attr of node.attributes) {
			if (attr.type === "JSXAttribute") {
				duplicates.addLocation(getAttributeKey(attr), attr.loc);
			}
		}

		duplicates.process();

		return node;
	},
};
