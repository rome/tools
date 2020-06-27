import {descriptions} from "@romejs/diagnostics";
import {JSXElement} from "@romejs/ast";
import {Path} from "@romejs/compiler";
import {hasJSXAttribute, isJSXElement} from "@romejs/js-ast-utils";

export default {
	name: "jsxA11YScope",

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
