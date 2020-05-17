import {descriptions} from "@romejs/diagnostics";
import {AnyNode} from "@romejs/js-ast";
import {Path} from "@romejs/js-compiler";
import {hasJSXAttribute, isJSXElement} from "@romejs/js-ast-utils";

export default {
	name: "jsxA11yScope",

	enter(path: Path): AnyNode {
		const {node} = path;

		if (
			isJSXElement(node) &&
			hasJSXAttribute(node, "scope") &&
			!isJSXElement(node, "th")
		) {
			path.context.addNodeDiagnostic(node, descriptions.LINT.REACT_JSX_NO_SCOPE);
		}

		return node;
	},
};
