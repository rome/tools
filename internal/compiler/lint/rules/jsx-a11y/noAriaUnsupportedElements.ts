import {createVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";
import {JSXElement} from "@internal/ast";
import {hasJSXAttribute, isJSXElement} from "@internal/js-ast-utils";
import {ARIAProperty, ariaPropsMap} from "@internal/compiler/lint/utils/aria";

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

export default createVisitor({
	name: "jsx-a11y/noAriaUnsupportedElements",
	enter(path) {
		const {node} = path;

		if (
			!(isJSXElement(node, "meta") ||
			isJSXElement(node, "html") ||
			isJSXElement(node, "script") ||
			isJSXElement(node, "style"))
		) {
			return signals.retain;
		}

		if (hasAriaAttributes(node)) {
			path.context.addNodeDiagnostic(
				node,
				descriptions.LINT.JSX_A11Y_ARIA_UNSUPPORTED_ELEMENTS,
			);
		}

		return signals.retain;
	},
});
