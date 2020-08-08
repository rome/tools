import {createVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";
import {JSXElement} from "@internal/ast";
import {hasJSXAttribute, isJSXElement} from "@internal/js-ast-utils";

function hasMuted(node: JSXElement): boolean {
	return hasJSXAttribute(node, "muted");
}

function hasTrack(node: JSXElement): boolean {
	return node.children.some((child) => isJSXElement(child, "track"));
}

export default createVisitor({
	name: "jsx-a11y/useMediaCaption",
	enter(path) {
		const {node} = path;

		if (!(isJSXElement(node, "video") || isJSXElement(node, "audio"))) {
			return signals.retain;
		}

		if (isJSXElement(node, "video") && hasMuted(node)) {
			return signals.retain;
		}

		if (!hasTrack(node)) {
			path.context.addNodeDiagnostic(
				node,
				descriptions.LINT.JSX_A11Y_MEDIA_HAS_CAPTION,
			);
		}

		return signals.retain;
	},
});
