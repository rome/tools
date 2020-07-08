import {descriptions} from "@romefrontend/diagnostics";
import {Path} from "@romefrontend/compiler";
import {getJSXAttribute, hasJSXAttribute} from "@romefrontend/js-ast-utils";

export default {
	name: "jsxA11YNoAccessKey",

	enter(path: Path) {
		const {node} = path;

		if (node.type === "JSXElement" && hasJSXAttribute(node, "accessKey")) {
			return path.context.addFixableDiagnostic(
				{
					target: getJSXAttribute(node, "accessKey"),
					old: node,
					fixed: {
						...node,
						attributes: node.attributes.filter((attribute) =>
							attribute.type !== "JSXAttribute" ||
							attribute.name.name !== "accessKey"
						),
					},
				},
				descriptions.LINT.JSX_A11Y_NO_ACCESS_KEY,
			);
		}

		return node;
	},
};
