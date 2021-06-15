import {createLintVisitor, signals} from "@internal/compiler";
import {HTMLElement, JSXElement} from "@internal/ast";
import {descriptions} from "@internal/diagnostics";
import {hasJSXAttribute, isJSXElement} from "@internal/js-ast-utils";
import isHTMLElement from "@internal/js-ast-utils/isHTMLElement";
import hasHTMLAttribute from "@internal/js-ast-utils/hasHTMLAttribute";

function hasAnchorContent(node: JSXElement): boolean {
	return (
		hasJSXAttribute(node, "dangerouslySetInnerHTML") ||
		(node.children.length > 0 &&
		node.children.some((child) =>
			child.type !== "JSXElement" || !hasJSXAttribute(child, "aria-hidden")
		))
	);
}

function htmlHasAnchorContent(node: HTMLElement) {
	return node.children.some((child) =>
		child.type !== "HTMLElement" || !hasHTMLAttribute(child, "aria-hidden")
	);
}

export default createLintVisitor({
	name: "a11y/useAnchorContent",
	enter(path) {
		const {node} = path;

		if (isHTMLElement(node) && node.name.name === "a") {
			if (!htmlHasAnchorContent(node)) {
				path.context.addNodeDiagnostic(
					node,
					descriptions.LINT.A11_Y_USE_ANCHOR_CONTENT,
				);
			}
		} else if (isJSXElement(node, "a") && !hasAnchorContent(node)) {
			path.context.addNodeDiagnostic(
				node,
				descriptions.LINT.A11_Y_USE_ANCHOR_CONTENT,
			);
		}

		return signals.retain;
	},
});
