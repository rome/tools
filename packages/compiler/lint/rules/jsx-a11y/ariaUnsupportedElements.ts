import {Path} from "@romefrontend/compiler";
import {TransformExitResult} from "@romefrontend/compiler/types";
import {descriptions} from "@romefrontend/diagnostics";
import {JSXElement} from "@romefrontend/ast";
import {hasJSXAttribute, isJSXElement} from "@romefrontend/js-ast-utils";
import {
	ARIAProperty,
	ariaPropsMap,
} from "@romefrontend/compiler/lint/utils/aria";

function hasAriaAttributes(node: JSXElement): boolean {
	const hasRole = hasJSXAttribute(node, "role");

	return (
		hasRole ||
		node.attributes.some((attr) =>
			attr.type === "JSXAttribute" &&
			attr.name.type === "JSXIdentifier" &&
			ariaPropsMap.has((attr.name.name as ARIAProperty))
		)
	);
}

export default {
	name: "jsx-a11y/ariaUnsupportedElements",
	enter(path: Path): TransformExitResult {
		const {node} = path;

		if (
			!(isJSXElement(node, "meta") ||
			isJSXElement(node, "html") ||
			isJSXElement(node, "script") ||
			isJSXElement(node, "style"))
		) {
			return node;
		}

		if (hasAriaAttributes(node)) {
			path.context.addNodeDiagnostic(
				node,
				descriptions.LINT.JSX_A11Y_ARIA_UNSUPPORTED_ELEMENTS,
			);
		}

		return node;
	},
};
