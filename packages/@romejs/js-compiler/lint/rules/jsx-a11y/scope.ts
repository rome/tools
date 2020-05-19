import {descriptions} from "@romejs/diagnostics";
import {AnyNode} from "@romejs/ast";
import {Path} from "@romejs/js-compiler";
import {hasJSXAttribute, isJSXElement} from "@romejs/js-ast-utils";

export default {
	name: "jsxA11YScope",

	enter(path: Path): AnyNode {
		const {node} = path;

		if (
			isJSXElement(node) &&
			hasJSXAttribute(node, "scope") &&
			!isJSXElement(node, "th")
		) {
			path.context.addNodeDiagnostic(node, descriptions.LINT.JSX_A11Y_NO_SCOPE);
		}

		return node;
	},
};
