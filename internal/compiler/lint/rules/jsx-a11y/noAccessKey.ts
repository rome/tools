import {descriptions} from "@romefrontend/diagnostics";
import {createVisitor, signals} from "@romefrontend/compiler";
import {getJSXAttribute, hasJSXAttribute} from "@romefrontend/js-ast-utils";

export default createVisitor({
	name: "jsx-a11y/noAccessKey",

	enter(path) {
		const {node} = path;

		if (node.type === "JSXElement" && hasJSXAttribute(node, "accessKey")) {
			return path.addFixableDiagnostic(
				{
					target: getJSXAttribute(node, "accessKey"),
					fixed: signals.replace({
						...node,
						attributes: node.attributes.filter((attribute) =>
							attribute.type !== "JSXAttribute" ||
							attribute.name.name !== "accessKey"
						),
					}),
				},
				descriptions.LINT.JSX_A11Y_NO_ACCESS_KEY,
			);
		}

		return signals.retain;
	},
});
