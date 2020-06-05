import {descriptions} from "@romejs/diagnostics";
import {AnyNode} from "@romejs/ast";
import {Path} from "@romejs/compiler";
import {getJSXAttribute, hasJSXAttribute} from "@romejs/js-ast-utils";

export default {
	name: "jsxA11YNoAutofocus",

	enter(path: Path): AnyNode {
		const {node} = path;

		if (node.type === "JSXElement" && hasJSXAttribute(node, "autoFocus")) {
			path.context.addFixableDiagnostic(
				{
					target: getJSXAttribute(node, "autoFocus"),
					old: node,
					fixed: {
						...node,
						attributes: node.attributes.filter((attribute) =>
							attribute.type === "JSXAttribute" &&
							attribute.name.name !== "autoFocus"
						),
					},
				},
				descriptions.LINT.JSX_A11Y_NO_AUTOFOCUS,
			);
		}

		return node;
	},
};
