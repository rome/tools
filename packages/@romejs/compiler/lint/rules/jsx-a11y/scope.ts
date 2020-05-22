import {descriptions} from "@romejs/diagnostics";
import {AnyNode, JSXElement} from "@romejs/ast";
import {Path} from "@romejs/compiler";
import {hasJSXAttribute, isJSXElement} from "@romejs/js-ast-utils";

export default {
	name: "jsxA11YScope",

	enter(path: Path): AnyNode {
		const {node} = path;
		const jsxNode = (node as JSXElement);

		if (
			isJSXElement(node) &&
			hasJSXAttribute(node, "scope") &&
			!isJSXElement(node, "th")
		) {
			path.context.addFixableDiagnostic(
				{
					old: jsxNode,
					fixed: {
						...jsxNode,
						attributes: jsxNode.attributes.filter((attribute) =>
							attribute.type === "JSXAttribute" &&
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
