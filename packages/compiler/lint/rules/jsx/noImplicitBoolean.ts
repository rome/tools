import {createVisitor, signals} from "@romefrontend/compiler";
import {descriptions} from "@romefrontend/diagnostics";
import {jsBooleanLiteral, jsxExpressionContainer} from "@romefrontend/ast";

export default createVisitor({
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
