import {descriptions} from "@romejs/diagnostics";
import {AnyNode} from "@romejs/ast";
import {Path} from "@romejs/compiler";
import {isJSXElement} from "@romejs/js-ast-utils";

function jsxAnchorHasBlankTarget(node: AnyNode) {
	return (
		isJSXElement(node, "a") &&
		node.attributes.some((attribute) =>
			attribute.type === "JSXAttribute" &&
			attribute.name.name === "target" &&
			attribute.value &&
			attribute.value.type === "JSStringLiteral" &&
			attribute.value.value === "_blank"
		)
	);
}

function jsxAnchorHasNoReferrer(node: AnyNode) {
	return (
		isJSXElement(node, "a") &&
		node.attributes.some((attribute) =>
			attribute.type === "JSXAttribute" &&
			attribute.name.name === "rel" &&
			attribute.value &&
			attribute.value.type === "JSStringLiteral" &&
			attribute.value.value.includes("noreferrer")
		)
	);
}

function jsxAnchorHasExternalLink(node: AnyNode) {
	return (
		isJSXElement(node, "a") &&
		node.attributes.some((attribute) =>
			attribute.type === "JSXAttribute" &&
			attribute.name.name === "href" &&
			attribute.value &&
			((attribute.value.type === "JSStringLiteral" &&
			/^(?:\w+:|\/\/)/.test(attribute.value.value)) ||
			attribute.value.type === "JSXExpressionContainer")
		)
	);
}

export default {
	name: "jsxA11YNoTargetBlank",

	enter(path: Path): AnyNode {
		const {node} = path;

		if (
			jsxAnchorHasBlankTarget(node) &&
			!jsxAnchorHasNoReferrer(node) &&
			jsxAnchorHasExternalLink(node)
		) {
			path.context.addNodeDiagnostic(
				node,
				descriptions.LINT.JSX_A11Y_NO_TARGET_BLANK,
			);
		}

		return node;
	},
};
