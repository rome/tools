import {descriptions} from "@romejs/diagnostics";
import {
	getJSXAttribute,
	hasJSXAttribute,
	isJSXElement,
} from "@romejs/js-ast-utils";
import {Path, TransformExitResult} from "@romejs/compiler";

export default {
	name: "noOnChange",
	enter(path: Path): TransformExitResult {
		const {context, node} = path;

		if (!isJSXElement(node, "select") && !isJSXElement(node, "option")) {
			return node;
		}

		if (!hasJSXAttribute(node, "onChange")) {
			return node;
		}

		if (hasJSXAttribute(node, "onBlur")) {
			return node;
		}

		context.addNodeDiagnostic(
			getJSXAttribute(node, "onChange"),
			descriptions.LINT.JSX_A11Y_NO_ON_CHANGE,
		);

		return node;
	},
};
