import {descriptions} from "@romejs/diagnostics";
import {AnyNode} from "@romejs/ast";
import {Path} from "@romejs/compiler";
import {hasJSXAttribute, isJSXElement} from "@romejs/js-ast-utils";

export default {
	name: "jsxA11YNoAutofocus",

	enter(path: Path): AnyNode {
		const {node} = path;

		if (isJSXElement(node) && hasJSXAttribute(node, "autoFocus")) {
			path.context.addNodeDiagnostic(
				node,
				descriptions.LINT.JSX_A11Y_NO_AUTOFOCUS,
			);
		}

		return node;
	},
};
