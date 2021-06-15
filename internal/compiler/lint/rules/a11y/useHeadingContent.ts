import {createLintVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";
import {hasJSXAttribute, isJSXElement} from "@internal/js-ast-utils";
import {HTMLElement, JSXElement} from "@internal/ast";
import isHTMLElement from "@internal/js-ast-utils/isHTMLElement";
import hasHTMLAttribute from "@internal/js-ast-utils/hasHTMLAttribute";

const HEADINGS = ["h1", "h2", "h3", "h4", "h5", "h6"];

function hasHeadingContent(node: JSXElement): boolean {
	if (!HEADINGS.some((heading) => isJSXElement(node, heading))) {
		return false;
	}
	return (
		hasJSXAttribute(node, "dangerouslySetInnerHTML") ||
		(node.children.length > 0 &&
		node.children.some((child) =>
			child.type !== "JSXElement" || !hasJSXAttribute(child, "aria-hidden")
		))
	);
}

function htmlHasHeadingContent(node: HTMLElement): boolean {
	return node.children.some((child) =>
		child.type !== "HTMLElement" || !hasHTMLAttribute(child, "aria-hidden")
	);
}

export default createLintVisitor({
	name: "a11y/useHeadingContent",
	enter(path) {
		const {node} = path;

		if (isHTMLElement(node) && HEADINGS.includes(node.name.name)) {
			if (!htmlHasHeadingContent(node)) {
				path.context.addNodeDiagnostic(
					node,
					descriptions.LINT.A11_Y_USE_HEADING_CONTENT,
				);
			}
		} else if (
			HEADINGS.some((heading) => isJSXElement(node, heading)) &&
			!hasHeadingContent(node as JSXElement)
		) {
			path.context.addNodeDiagnostic(
				node,
				descriptions.LINT.A11_Y_USE_HEADING_CONTENT,
			);
		}

		return signals.retain;
	},
});
