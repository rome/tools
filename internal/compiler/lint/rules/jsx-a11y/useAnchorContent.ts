import {createVisitor, signals} from "@internal/compiler";
import {JSXElement} from "@internal/ast";
import {descriptions} from "@internal/diagnostics";
import {hasJSXAttribute, isJSXElement} from "@internal/js-ast-utils";

function hasAnchorContent(node: JSXElement): boolean {
	return (
		hasJSXAttribute(node, "dangerouslySetInnerHTML") ||
		(node.children.length > 0 &&
		node.children.some((child) =>
			child.type !== "JSXElement" || !hasJSXAttribute(child, "aria-hidden")
		))
	);
}

export default createVisitor({
	name: "jsx-a11y/useAnchorContent",
	enter(path) {
		const {node} = path;

		if (isJSXElement(node, "a") && !hasAnchorContent(node)) {
			path.context.addNodeDiagnostic(
				node,
				descriptions.LINT.JSX_A11Y_ANCHOR_HAS_CONTENT,
			);
		}

		return signals.retain;
	},
});
