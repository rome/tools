import {descriptions} from "@romejs/diagnostics";
import {AnyNode} from "@romejs/js-ast";
import {Path} from "@romejs/js-compiler";
import {hasJSXAttribute, isJSXElement} from "@romejs/js-ast-utils";

export default {
	name: "jsxA11YNoAccessKey",

	enter(path: Path): AnyNode {
		const {node} = path;

		if (isJSXElement(node) && hasJSXAttribute(node, "accessKey")) {
			path.context.addNodeDiagnostic(
				node,
				descriptions.LINT.JSX_A11Y_NO_ACCESS_KEY,
			);
		}

		return node;
	},
};
