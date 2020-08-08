import {descriptions} from "@internal/diagnostics";
import {createVisitor, signals} from "@internal/compiler";
import {doesNodeMatchPattern, hasJSXAttribute} from "@internal/js-ast-utils";

export default createVisitor({
	name: "jsx-a11y/noHeaderScope",

	enter(path) {
		const {node} = path;

		if (
			node.type === "JSXElement" &&
			hasJSXAttribute(node, "scope") &&
			!doesNodeMatchPattern(node.name, "th")
		) {
			return path.addFixableDiagnostic(
				{
					fixed: signals.replace({
						...node,
						attributes: node.attributes.filter((attribute) =>
							attribute.type !== "JSXAttribute" ||
							attribute.name.name !== "scope"
						),
					}),
				},
				descriptions.LINT.JSX_A11Y_NO_SCOPE,
			);
		}

		return signals.retain;
	},
});
