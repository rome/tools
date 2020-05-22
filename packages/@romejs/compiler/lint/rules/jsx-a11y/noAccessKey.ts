import {descriptions} from "@romejs/diagnostics";
import {AnyNode} from "@romejs/ast";
import {Path} from "@romejs/compiler";
import {
	getJSXAttribute,
	hasJSXAttribute,
	isJSXElement,
} from "@romejs/js-ast-utils";

export default {
	name: "jsxA11YNoAccessKey",

	enter(path: Path): AnyNode {
		const {node} = path;

		if (isJSXElement(node) && hasJSXAttribute(node, "accessKey")) {
			path.context.addFixableDiagnostic(
				{
					target: getJSXAttribute(node, "accessKey"),
					old: node,
					fixed: {
						...node,
						attributes: node.attributes.filter((attribute) =>
							attribute.type === "JSXAttribute" &&
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
