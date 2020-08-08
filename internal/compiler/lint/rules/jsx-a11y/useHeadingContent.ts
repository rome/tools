import {createVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";
import {hasJSXAttribute, isJSXElement} from "@internal/js-ast-utils";
import {JSXElement} from "@internal/ast";

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

export default createVisitor({
	name: "jsx-a11y/useHeadingContent",
	enter(path) {
		const {node} = path;

		if (
			HEADINGS.some((heading) => isJSXElement(node, heading)) &&
			!hasHeadingContent((node as JSXElement))
		) {
			path.context.addNodeDiagnostic(
				node,
				descriptions.LINT.JSX_A11Y_HEADING_HAS_CONTENT,
			);
		}

		return signals.retain;
	},
});
