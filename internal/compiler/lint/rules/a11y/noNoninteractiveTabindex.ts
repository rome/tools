import {createLintVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";
import {
	getJSXAttribute,
	getJSXElementName,
	hasJSXAttribute,
} from "@internal/js-ast-utils";
import {HTMLAttribute, JSXAttribute} from "@internal/ast";
import {
	ariaRolesMap,
	elementsToConceptsMap,
	isRoleInteractive,
} from "@internal/compiler/lint/utils/aria";
import {isJSXDOMElement} from "@internal/js-ast-utils/isJSXDOMElement";
import isHTMLElement from "@internal/js-ast-utils/isHTMLElement";
import hasHTMLAttribute from "@internal/js-ast-utils/hasHTMLAttribute";
import getHTMLAttribute from "@internal/js-ast-utils/getHTMLAttribute";

function hasValidTabIndexValue(
	node: JSXAttribute | HTMLAttribute | undefined,
): number | undefined {
	if (!node) {
		return undefined;
	}

	if (node.type === "HTMLAttribute" && node.value) {
		return Number(node.value.value);
	} else {
		if (node?.value?.type === "JSStringLiteral") {
			const value = Number(node.value.value);
			if (value < 1) {
				return value;
			}
		}
		if (node?.value?.type === "JSXExpressionContainer") {
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
	}

	return undefined;
}

export default createLintVisitor({
	name: "a11y/noNoninteractiveTabindex",
	enter(path) {
		const {node} = path;

		if (isHTMLElement(node)) {
			if (!hasHTMLAttribute(node, "tabindex")) {
				return signals.retain;
			}

			const tabIndexAttribute = getHTMLAttribute(node, "tabindex");
			const tabIndexValue = hasValidTabIndexValue(tabIndexAttribute);
			if (tabIndexValue !== undefined) {
				const elementName = node.name.name;
				/**
				 * Check if:
				 * 1. element doesn't have the role attribute
				 * 2. element is not interactive
				 *
				 * e.g. <div tabIndex="0"></div>
				 */
				if (!hasHTMLAttribute(node, "role")) {
					const elementToRole = elementsToConceptsMap.get(elementName);
					// the element is not part of any role, so it's an error
					// e.g. div, span, etc.
					if (elementToRole) {
						// not having the "widget" role means that the role is not interactive
						if (!elementToRole.has("widget") && tabIndexValue > -1) {
							path.context.addNodeDiagnostic(
								tabIndexAttribute,
								descriptions.LINT.A11_Y_NO_NONINTERACTIVE_TABINDEX,
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
								tabIndexAttribute,
								descriptions.LINT.A11_Y_NO_NONINTERACTIVE_TABINDEX,
							);
						}
					}
				}
				const attr = getHTMLAttribute(node, "role");
				if (attr?.value) {
					const role = ariaRolesMap.get(attr.value.value);
					if (role) {
						/**
						 * Some roles are not interactive (e.g. article),
						 * which means that they can have a tabIndex=-1 which removes the focus behaviour
						 *
						 * e.g. <article tabIndex="-1" /> is a valid statement
						 */
						if (!isRoleInteractive(role) && tabIndexValue > -1) {
							path.context.addNodeDiagnostic(
								attr,
								descriptions.LINT.A11_Y_NO_NONINTERACTIVE_TABINDEX,
							);
						}
					}
				}
			}
		} else if (isJSXDOMElement(node)) {
			// not tabIndex, no worth continuing
			if (!hasJSXAttribute(node, "tabIndex")) {
				return signals.retain;
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
					const elementToRole = elementsToConceptsMap.get(elementName);
					// the element is not part of any role, so it's an error
					// e.g. div, span, etc.
					if (elementToRole) {
						// not having the "widget" role means that the role is not interactive
						if (!elementToRole.has("widget") && tabIndexValue > -1) {
							path.context.addNodeDiagnostic(
								tabIndexAttribute,
								descriptions.LINT.A11_Y_NO_NONINTERACTIVE_TABINDEX,
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
								tabIndexAttribute,
								descriptions.LINT.A11_Y_NO_NONINTERACTIVE_TABINDEX,
							);
						}
					}
				}
				const attr = getJSXAttribute(node, "role");
				if (attr?.value?.type === "JSStringLiteral") {
					const role = ariaRolesMap.get(attr.value.value);
					if (role) {
						/**
						 * Some roles are not interactive (e.g. article),
						 * which means that they can have a tabIndex=-1 which removes the focus behaviour
						 *
						 * e.g. <article tabIndex="-1" /> is a valid statement
						 */
						if (!isRoleInteractive(role) && tabIndexValue > -1) {
							path.context.addNodeDiagnostic(
								attr,
								descriptions.LINT.A11_Y_NO_NONINTERACTIVE_TABINDEX,
							);
						}
					}
				}
			}
		}

		return signals.retain;
	},
});
