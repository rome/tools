import {Path, TransformExitResult} from "@romejs/js-compiler";
import {descriptions} from "@romejs/diagnostics";
import {
	getJSXAttribute,
	hasJSXAttribute,
	isJSXElement,
} from "@romejs/js-ast-utils";

export default {
	name: "tabindexNoPositive",
	enter(path: Path): TransformExitResult {
		const {node} = path;

		if (isJSXElement(node) && hasJSXAttribute(node, "tabIndex")) {
			const attribute = getJSXAttribute(node, "tabIndex");
			if (
				attribute &&
				attribute.value &&
				attribute.value.type === "JSStringLiteral"
			) {
				const tabIndexValue = attribute.value.value;
				if (Number(tabIndexValue) > 0) {
					path.context.addNodeDiagnostic(
						node,
						descriptions.LINT.JSX_A11Y_TABINDEX_NO_POSITIVE,
					);
				}
			}

			if (
				attribute &&
				attribute.value &&
				attribute.value.type === "JSXExpressionContainer"
			) {
				const expression = attribute.value.expression;
				if (expression.type === "JSNumericLiteral") {
					const tabIndexValue = expression.value;
					if (Number(tabIndexValue) > 0) {
						path.context.addNodeDiagnostic(
							node,
							descriptions.LINT.JSX_A11Y_TABINDEX_NO_POSITIVE,
						);
					}
				}
			}
		}
		return node;
	},
};
