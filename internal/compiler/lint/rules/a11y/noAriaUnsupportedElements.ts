import {createLintVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";
import {JSXElement} from "@internal/ast";
import {hasJSXAttribute, isJSXElement} from "@internal/js-ast-utils";
import {ARIAProperty, ariaPropsMap} from "@internal/compiler/lint/utils/aria";
import isHTMLElement from "@internal/js-ast-utils/isHTMLElement";
import getHTMLAttribute from "@internal/js-ast-utils/getHTMLAttribute";

function hasAriaAttributes(node: JSXElement): boolean {
	const hasRole = hasJSXAttribute(node, "role");

	return (
		hasRole ||
		node.attributes.some((attr) =>
			attr.type === "JSXAttribute" &&
			attr.name.type === "JSXIdentifier" &&
			ariaPropsMap.has(attr.name.name as ARIAProperty)
		)
	);
}

const ELEMENTS_TO_CHECK = new Set(["meta", "html", "script", "style"]);

export default createLintVisitor({
	name: "a11y/noAriaUnsupportedElements",
	enter(path) {
		const {node} = path;

		if (isHTMLElement(node)) {
			if (ELEMENTS_TO_CHECK.has(node.name.name)) {
				const role = getHTMLAttribute(node, "role");

				if (
					role ||
					node.attributes.some((attr) =>
						ariaPropsMap.has(attr.name.name as ARIAProperty)
					)
				) {
					path.context.addNodeDiagnostic(
						node,
						descriptions.LINT.A11_Y_NO_ARIA_UNSUPPORTED_ELEMENTS,
					);
				}
			}
			return signals.retain;
		} else if (node.type === "JSXElement") {
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
					descriptions.LINT.A11_Y_NO_ARIA_UNSUPPORTED_ELEMENTS,
				);
			}
		}

		return signals.retain;
	},
});
