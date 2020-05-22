import {Path, TransformExitResult} from "@romejs/compiler";
import {JSXElement} from "@romejs/ast";
import {descriptions} from "@romejs/diagnostics";
import {hasJSXAttribute, isJSXElement} from "@romejs/js-ast-utils";

function hasAnchorContent(node: JSXElement): boolean {
	return (
		hasJSXAttribute(node, "dangerouslySetInnerHTML") ||
		(node.children.length > 0 &&
		node.children.some((child) =>
			child.type !== "JSXElement" || !hasJSXAttribute(child, "aria-hidden")
		))
	);
}

export default {
	name: "jsxA11YAnchorHasContent",
	enter(path: Path): TransformExitResult {
		const {node} = path;
		if (isJSXElement(node, "a") && !hasAnchorContent(node)) {
			path.context.addNodeDiagnostic(
				node,
				descriptions.LINT.JSX_A11Y_ANCHOR_HAS_CONTENT,
			);
		}

		return node;
	},
};
