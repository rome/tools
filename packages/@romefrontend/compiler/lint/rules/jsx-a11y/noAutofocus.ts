import {descriptions} from "@romefrontend/diagnostics";
import {Path} from "@romefrontend/compiler";
import {getJSXAttribute, hasJSXAttribute} from "@romefrontend/js-ast-utils";

export default {
	name: "jsxA11YNoAutofocus",

	enter(path: Path) {
		const {node} = path;

		if (node.type === "JSXElement" && hasJSXAttribute(node, "autoFocus")) {
			return path.context.addFixableDiagnostic(
				{
					target: getJSXAttribute(node, "autoFocus"),
					old: node,
					fixed: {
						...node,
						attributes: node.attributes.filter((attribute) =>
							attribute.type !== "JSXAttribute" ||
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
