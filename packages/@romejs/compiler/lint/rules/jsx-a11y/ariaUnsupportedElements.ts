import {Path} from "@romejs/compiler";
import {TransformExitResult} from "@romejs/compiler/types";
import {descriptions} from "@romejs/diagnostics";
import {JSXElement} from "@romejs/ast";
import {hasJSXAttribute, isJSXElement} from "@romejs/js-ast-utils";
import {ARIAProperty, ariaPropsMap} from "../../utils/aria";

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
	name: "ariaUnsupportedElements",
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
