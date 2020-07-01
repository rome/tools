import {Path, TransformExitResult} from "@romejs/compiler";
import {descriptions} from "@romejs/diagnostics";
import {DiagnosticsDuplicateHelper} from "@romejs/compiler/lib/DiagnosticsDuplicateHelper";
import {JSXAttribute} from "@romejs/ast";

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
