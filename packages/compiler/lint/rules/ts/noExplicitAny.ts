import {createVisitor, signals} from "@romefrontend/compiler";
import {descriptions} from "@romefrontend/diagnostics";

export default createVisitor({
	name: "ts/noExplicitAny",
	enter(path) {
		const {context, node} = path;

		if (node.type === "TSAnyKeywordTypeAnnotation") {
			context.addNodeDiagnostic(node, descriptions.LINT.TS_NO_EXPLICIT_ANY);
		}

		return signals.retain;
	},
});
