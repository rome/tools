import {createVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";

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
