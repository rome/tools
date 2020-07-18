import {Path, TransformExitResult} from "@romefrontend/compiler";
import {JSXElement} from "@romefrontend/ast";
import {descriptions} from "@romefrontend/diagnostics";
import {hasJSXAttribute, isJSXElement} from "@romefrontend/js-ast-utils";

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
	name: "jsx-a11y/anchorHasContent",
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
