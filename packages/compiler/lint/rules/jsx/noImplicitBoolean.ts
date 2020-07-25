import {Path, TransformExitResult} from "@romefrontend/compiler";
import {descriptions} from "@romefrontend/diagnostics";
import {jsBooleanLiteral, jsxExpressionContainer} from "@romefrontend/ast";

export default {
	name: "jsx/noImplicitBoolean",
	enter(path: Path): TransformExitResult {
		const {context, node} = path;

		if (node.type === "JSXAttribute" && !node.value) {
			return context.addFixableDiagnostic(
				{
					old: node,
					fixed: {
						...node,
						value: jsxExpressionContainer.create({
							expression: jsBooleanLiteral.quick(true),
						}),
					},
				},
				descriptions.LINT.JSX_NO_IMPLICIT_BOOLEAN,
			);
		}

		return node;
	},
};
