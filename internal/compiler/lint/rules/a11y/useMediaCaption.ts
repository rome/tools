import {createLintVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";
import {HTMLElement, JSXElement} from "@internal/ast";
import {hasJSXAttribute, isJSXElement} from "@internal/js-ast-utils";
import {isJSXDOMElement} from "@internal/js-ast-utils/isJSXDOMElement";
import isHTMLElement from "@internal/js-ast-utils/isHTMLElement";
import getHTMLAttribute from "@internal/js-ast-utils/getHTMLAttribute";

function hasMuted(node: JSXElement): boolean {
	return hasJSXAttribute(node, "muted");
}

function hasTrack(node: JSXElement | HTMLElement): boolean {
	if (isHTMLElement(node)) {
		return node.children.some((child) =>
			isHTMLElement(child) && child.name.name === "track"
		);
	}
	return node.children.some((child) => isJSXElement(child, "track"));
}

export default createLintVisitor({
	name: "a11y/useMediaCaption",
	enter(path) {
		const {node} = path;
		if (isHTMLElement(node)) {
			if (node.name.name === "video" || node.name.name === "audio") {
				if (node.name.name === "video") {
					const muted = getHTMLAttribute(node, "muted", true);
					if (muted !== undefined) {
						return signals.retain;
					}
				}
				if (!hasTrack(node)) {
					path.context.addNodeDiagnostic(
						node,
						descriptions.LINT.A11_Y_USE_MEDIA_CAPTION,
					);
				}
			}
		} else {
			if (!isJSXDOMElement(node)) {
				return signals.retain;
			}

			if (!(isJSXElement(node, "video") || isJSXElement(node, "audio"))) {
				return signals.retain;
			}

			if (isJSXElement(node, "video") && hasMuted(node)) {
				return signals.retain;
			}

			if (!hasTrack(node)) {
				path.context.addNodeDiagnostic(
					node,
					descriptions.LINT.A11_Y_USE_MEDIA_CAPTION,
				);
			}
		}

		return signals.retain;
	},
});
