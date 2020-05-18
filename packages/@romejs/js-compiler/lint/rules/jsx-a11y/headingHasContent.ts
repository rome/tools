import {Path, TransformExitResult} from "@romejs/js-compiler";
import {descriptions} from "@romejs/diagnostics";
import {hasJSXAttribute, isJSXElement} from "@romejs/js-ast-utils";
import {AnyNode} from "@romejs/js-ast";

const HEADINGS = ["h1", "h2", "h3", "h4", "h5", "h6"];

function isInvalidHeading(node: AnyNode) {
	if (!isJSXElement(node)) {
		return false;
	}

	return (
		HEADINGS.some((heading) => isJSXElement(node, heading)) &&
		((node.children.length === 0 &&
		!hasJSXAttribute(node, "dangerouslySetInnerHTML")) ||
		(node.children.length > 0 &&
		isJSXElement(node.children[0]) &&
		hasJSXAttribute(node.children[0], "aria-hidden")))
	);
}

export default {
	name: "jsxA11YHeadingHasContent",
	enter(path: Path): TransformExitResult {
		const {node} = path;

		if (isInvalidHeading(node)) {
			path.context.addNodeDiagnostic(
				node,
				descriptions.LINT.JSX_A11Y_HEADING_HAS_CONTENT,
			);
		}

		return node;
	},
};
