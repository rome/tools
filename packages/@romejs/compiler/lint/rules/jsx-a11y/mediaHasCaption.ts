import {Path, TransformExitResult} from "@romejs/compiler";
import {descriptions} from "@romejs/diagnostics";
import {JSXElement} from "@romejs/ast";
import {hasJSXAttribute, isJSXElement} from "@romejs/js-ast-utils";

function hasMuted(node: JSXElement): boolean {
	return hasJSXAttribute(node, "muted");
}

function hasTrack(node: JSXElement): boolean {
	return node.children.some((child) => isJSXElement(child, "track"));
}

export default {
	name: "mediaHasCaption",
	enter(path: Path): TransformExitResult {
		const {node} = path;

		if (!(isJSXElement(node, "video") || isJSXElement(node, "audio"))) {
			return node;
		}

		if (isJSXElement(node, "video") && hasMuted(node)) {
			return node;
		}

		if (!hasTrack(node)) {
			path.context.addNodeDiagnostic(
				node,
				descriptions.LINT.JSX_A11Y_MEDIA_HAS_CAPTION,
			);
		}

		return node;
	},
};
