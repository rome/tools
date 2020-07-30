import {createVisitor, signals} from "@romefrontend/compiler";
import {descriptions} from "@romefrontend/diagnostics";
import {DiagnosticsDuplicateHelper} from "@romefrontend/compiler/lib/DiagnosticsDuplicateHelper";
import {JSXAttribute} from "@romefrontend/ast";

function getAttributeKey(node: JSXAttribute): string {
	const name = node.name.name;
	return typeof name === "string" ? name : name.name;
}

export default createVisitor({
	name: "jsx/noDuplicateProps",

	enter(path) {
		const {context, node} = path;

		if (node.type !== "JSXElement") {
			return signals.retain;
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

		return signals.retain;
	},
});
