import {createLintVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";
import {jsBooleanLiteral, jsxExpressionContainer} from "@internal/ast";

export default createLintVisitor({
	name: "jsx/noImplicitBoolean",
	enter(path) {
		const {node} = path;

		if (node.type === "JSXAttribute" && !node.value) {
			return path.addFixableDiagnostic(
				{
					fixed: signals.replace({
						...node,
						value: jsxExpressionContainer.create({
							expression: jsBooleanLiteral.quick(true),
						}),
					}),
				},
				descriptions.LINT.JSX_NO_IMPLICIT_BOOLEAN,
			);
		}

		return signals.retain;
	},
});
