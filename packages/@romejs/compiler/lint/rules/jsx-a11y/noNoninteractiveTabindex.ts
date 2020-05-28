import {Path, TransformExitResult} from "@romejs/compiler";
import {descriptions} from "@romejs/diagnostics";
import {
	getJSXAttribute,
	hasJSXAttribute,
	isJSXElement,
} from "@romejs/js-ast-utils";
import {
	elementsToConcepts,
	isRoleInteractive,
	roles,
} from "@romejs/compiler/lint/rules/ariaHelpers";
import getJSXElementName from "@romejs/js-ast-utils/getJSXElementName";
import {JSXAttribute} from "@romejs/ast";

function hasValidTabIndexValue(
	node: JSXAttribute | undefined,
): number | undefined {
	if (node && node.value && node.value.type === "JSStringLiteral") {
		const value = Number(node.value.value);
		if (value < 1) {
			return value;
		}
	}
	if (node && node.value && node.value.type === "JSXExpressionContainer") {
		const expression = node.value.expression;

		if (
			expression.type === "JSNumericLiteral" ||
			expression.type === "JSStringLiteral"
		) {
			const tabIndexValue = expression.value;
			if (Number(tabIndexValue) < 1) {
				return Number(tabIndexValue);
			}
		}

		// for tabIndex={-1}
		if (expression.type === "JSUnaryExpression") {
			return -1;
		}
	}

	return undefined;
}

export default {
	name: "jsxA11YNoNoninteractiveTabindex",
	enter(path: Path): TransformExitResult {
		const {node} = path;

		if (isJSXElement(node)) {
			// it's a component, we don't know how the tabIndex is handled
			if (node.name && node.name.type === "JSXReferenceIdentifier") {
				return node;
			}

			// not tabIndex, no worth continuing
			if (!hasJSXAttribute(node, "tabIndex")) {
				return node;
			}

			const tabIndexAttribute = getJSXAttribute(node, "tabIndex");
			const tabIndexValue = hasValidTabIndexValue(tabIndexAttribute);
			if (tabIndexValue !== undefined) {
				const elementName = getJSXElementName(node);
				/**
				 * Check if:
				 * 1. element doesn't have the role attribute
				 * 2. element is not interactive
				 *
				 * e.g. <div tabIndex="0"></div>
				 */
				if (!hasJSXAttribute(node, "role")) {
					const elementToRole = elementsToConcepts.get(elementName);
					// the element is not part of any role, so it's an error
					// e.g. div, span, etc.
					if (elementToRole) {
						// not having the "widget" role means that the role is not interactive
						if (!elementToRole.has("widget") && tabIndexValue > -1) {
							path.context.addNodeDiagnostic(
								node,
								descriptions.LINT.JSX_A11Y_NO_NONINTERACTIVE_TABINDEX,
							);
						}
					} else {
						/**
						 * Here we need to check the value of the index because
						 * <div tabIndex="-1" ></div>
						 * is a valid statement
						 */
						if (tabIndexValue > -1) {
							path.context.addNodeDiagnostic(
								node,
								descriptions.LINT.JSX_A11Y_NO_NONINTERACTIVE_TABINDEX,
							);
						}
					}
				}
				const attr = getJSXAttribute(node, "role");
				if (attr && attr.value && attr.value.type === "JSStringLiteral") {
					const role = roles.get(attr.value.value);
					if (role) {
						/**
						 * Some roles are not interactive (e.g. article),
						 * which means that they can have a tabIndex=-1 which removes the focus behaviour
						 *
						 * e.g. <article tabIndex="-1" /> is a valid statement
						 */
						if (!isRoleInteractive(role) && tabIndexValue > -1) {
							path.context.addNodeDiagnostic(
								node,
								descriptions.LINT.JSX_A11Y_NO_NONINTERACTIVE_TABINDEX,
							);
						}
					}
				}
			}
		}

		return node;
	},
};
