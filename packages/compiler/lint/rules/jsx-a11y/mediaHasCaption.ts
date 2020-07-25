import {Path, TransformExitResult} from "@romefrontend/compiler";
import {descriptions} from "@romefrontend/diagnostics";
import {JSXElement} from "@romefrontend/ast";
import {hasJSXAttribute, isJSXElement} from "@romefrontend/js-ast-utils";

function hasMuted(node: JSXElement): boolean {
	return hasJSXAttribute(node, "muted");
}

function hasTrack(node: JSXElement): boolean {
	return node.children.some((child) => isJSXElement(child, "track"));
}

export default {
	name: "jsx-a11y/mediaHasCaption",
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
