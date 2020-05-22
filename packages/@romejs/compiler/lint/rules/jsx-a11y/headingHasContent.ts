import {Path, TransformExitResult} from "@romejs/compiler";
import {descriptions} from "@romejs/diagnostics";
import {hasJSXAttribute, isJSXElement} from "@romejs/js-ast-utils";
import {JSXElement} from "@romejs/ast";

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

export default {
	name: "jsxA11YHeadingHasContent",
	enter(path: Path): TransformExitResult {
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

		return node;
	},
};
