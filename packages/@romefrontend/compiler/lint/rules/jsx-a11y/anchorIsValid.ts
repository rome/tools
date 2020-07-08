import {Path, TransformExitResult} from "@romefrontend/compiler";
import {descriptions} from "@romefrontend/diagnostics";
import {
	getJSXAttribute,
	hasJSXAttribute,
	isJSXElement,
} from "@romefrontend/js-ast-utils";
import {JSXAttribute} from "@romefrontend/ast";

function hrefValue(attr: JSXAttribute | undefined, value: string): boolean {
	if (attr === undefined) {
		return false;
	}
	return (
		((attr?.value)?.type === "JSStringLiteral" && attr.value.value === value) ||
		((attr?.value)?.type === "JSXExpressionContainer" &&
		attr.value.expression.type === "JSStringLiteral" &&
		attr.value.expression.value === value) ||
		((attr?.value)?.type === "JSXExpressionContainer" &&
		attr.value.expression.type === "JSTemplateLiteral" &&
		attr.value.expression.quasis.some((element) => element.raw === value))
	);
}

function falsyHref(attr: JSXAttribute | undefined): boolean {
	if (
		attr === undefined ||
		(attr.value?.type === "JSXExpressionContainer" &&
		attr.value.expression.type === "JSNullLiteral")
	) {
		return true;
	}
	return false;
}

export default {
	name: "jsxA11YAnchorIsValid",
	enter(path: Path): TransformExitResult {
		const {node} = path;

		if (isJSXElement(node, "a")) {
			const attr = getJSXAttribute(node, "href");

			if (falsyHref(attr) && !hasJSXAttribute(node, "onClick")) {
				path.context.addNodeDiagnostic(
					node,
					descriptions.LINT.JSX_A11Y_ANCHOR_IS_VALID(
						"Provide a <emphasis>href</emphasis> attribute for the <emphasis>a</emphasis> element.",
					),
				);
			}

			if (attr && !hasJSXAttribute(node, "onClick")) {
				if (hrefValue(attr, "#") || hrefValue(attr, "javascript:void(0)")) {
					path.context.addNodeDiagnostic(
						node,
						descriptions.LINT.JSX_A11Y_ANCHOR_IS_VALID(
							"Provide a valid <emphasis>href</emphasis> attribute for the <emphasis>a</emphasis> element.",
						),
					);
				}
			}

			if (
				(hasJSXAttribute(node, "onClick") && !hasJSXAttribute(node, "href")) ||
				(hasJSXAttribute(node, "href") && hrefValue(attr, "#")) ||
				hrefValue(attr, "javascript:void(0)")
			) {
				path.context.addNodeDiagnostic(
					node,
					descriptions.LINT.JSX_A11Y_ANCHOR_IS_VALID(
						"Use a <emphasis>button</emphasis> element instead of an <emphasis>a</emphasis> element.",
					),
				);
			}
		}

		return node;
	},
};
