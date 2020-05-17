import {descriptions} from "@romejs/diagnostics";
import {AnyNode} from "@romejs/js-ast";
import {Path} from "@romejs/js-compiler";
import {hasJSXAttribute, isJSXElement} from "@romejs/js-ast-utils";

export default {
	name: "jsxA11yNoAutofocus",

	enter(path: Path): AnyNode {
		const {node} = path;

		if (isJSXElement(node) && hasJSXAttribute(node, "autoFocus")) {
			path.context.addNodeDiagnostic(
				node,
				descriptions.LINT.REACT_JSX_NO_AUTOFOCUS,
			);
		}

		return node;
	},
};
