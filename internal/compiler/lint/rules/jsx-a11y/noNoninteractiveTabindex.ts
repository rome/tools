import {createVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";
import {
	getJSXAttribute,
	getJSXElementName,
	hasJSXAttribute,
} from "@internal/js-ast-utils";
import {JSXAttribute} from "@internal/ast";
import {
	ariaRolesMap,
	elementsToConceptsMap,
	isRoleInteractive,
} from "@internal/compiler/lint/utils/aria";

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

export default createVisitor({
	name: "jsx-a11y/noNoninteractiveTabindex",
	enter(path) {
		const {node} = path;

		if (node.type === "JSXElement") {
			// it's a component, we don't know how the tabIndex is handled
			if (node.name && node.name.type === "JSXReferenceIdentifier") {
				return signals.retain;
			}

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
								tabIndexAttribute,
								descriptions.LINT.JSX_A11Y_NO_NONINTERACTIVE_TABINDEX,
							);
						}
					}
				}
				const attr = getJSXAttribute(node, "role");
				if (attr && attr.value && attr.value.type === "JSStringLiteral") {
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
								descriptions.LINT.JSX_A11Y_NO_NONINTERACTIVE_TABINDEX,
							);
						}
					}
				}
			}
		}

		return signals.retain;
	},
});
