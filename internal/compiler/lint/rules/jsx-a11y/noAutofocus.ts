import {descriptions} from "@internal/diagnostics";
import {createVisitor, signals} from "@internal/compiler";
import {getJSXAttribute, hasJSXAttribute} from "@internal/js-ast-utils";

export default createVisitor({
	name: "jsx-a11y/noAutofocus",

	enter(path) {
		const {node} = path;

		if (node.type === "JSXElement" && hasJSXAttribute(node, "autoFocus")) {
			return path.addFixableDiagnostic(
				{
					target: getJSXAttribute(node, "autoFocus"),
					fixed: signals.replace({
						...node,
						attributes: node.attributes.filter((attribute) =>
							attribute.type !== "JSXAttribute" ||
							attribute.name.name !== "autoFocus"
						),
					}),
				},
				descriptions.LINT.JSX_A11Y_NO_AUTOFOCUS,
			);
		}

		return signals.retain;
	},
});
