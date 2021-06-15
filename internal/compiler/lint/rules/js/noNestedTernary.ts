import {createLintVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";

export default createLintVisitor({
	name: "js/noNestedTernary",
	enter(path) {
		const {node} = path;

		if (node.type === "JSConditionalExpression") {
			if (node.alternate.type === "JSConditionalExpression") {
				path.context.addNodeDiagnostic(
					node.alternate,
					descriptions.LINT.JS_NO_NESTED_TERNARY,
				);
			}
			if (node.consequent.type === "JSConditionalExpression") {
				path.context.addNodeDiagnostic(
					node.consequent,
					descriptions.LINT.JS_NO_NESTED_TERNARY,
				);
			}
		}

		return signals.retain;
	},
});
