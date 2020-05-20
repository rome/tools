import {descriptions} from "@romejs/diagnostics";
import {hasJSXAttribute, isJSXElement} from "@romejs/js-ast-utils";
import {Path, TransformExitResult} from "@romejs/js-compiler";

export default {
	name: "noOnchange",
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

		context.addNodeDiagnostic(node, descriptions.LINT.JSX_A11Y_NO_ONCHANGE);

		return node;
	},
};
