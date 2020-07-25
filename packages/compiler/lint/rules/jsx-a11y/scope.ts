import {descriptions} from "@romefrontend/diagnostics";
import {JSXElement} from "@romefrontend/ast";
import {Path} from "@romefrontend/compiler";
import {hasJSXAttribute, isJSXElement} from "@romefrontend/js-ast-utils";

export default {
	name: "jsx-a11y/scope",

	enter(path: Path) {
		const {node} = path;
		const jsxNode = (node as JSXElement);

		if (
			node.type === "JSXElement" &&
			hasJSXAttribute(node, "scope") &&
			!isJSXElement(node, "th")
		) {
			return path.context.addFixableDiagnostic(
				{
					old: jsxNode,
					fixed: {
						...jsxNode,
						attributes: jsxNode.attributes.filter((attribute) =>
							attribute.type !== "JSXAttribute" ||
							attribute.name.name !== "scope"
						),
					},
				},
				descriptions.LINT.JSX_A11Y_NO_SCOPE,
			);
		}

		return node;
	},
};
