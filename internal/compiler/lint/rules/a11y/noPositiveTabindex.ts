import {CompilerPath, createLintVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";
import {getJSXAttribute, hasJSXAttribute} from "@internal/js-ast-utils";
import {
	HTMLAttribute,
	HTMLElement,
	JSXAttribute,
	JSXElement,
} from "@internal/ast";
import {isJSXDOMElement} from "@internal/js-ast-utils/isJSXDOMElement";
import isHTMLElement from "@internal/js-ast-utils/isHTMLElement";
import hasHTMLAttribute from "@internal/js-ast-utils/hasHTMLAttribute";
import getHTMLAttribute from "@internal/js-ast-utils/getHTMLAttribute";

function createHTMLDiagnostic(
	path: CompilerPath,
	node: HTMLElement,
	attribute: HTMLAttribute,
) {
	return path.addFixableDiagnostic(
		{
			target: attribute,
			fixed: signals.replace({
				...node,
				attributes: node.attributes.filter((attribute) =>
					attribute.type !== "HTMLAttribute" ||
					attribute.name.name !== "tabindex"
				),
			}),
		},
		descriptions.LINT.A11_Y_NO_POSITIVE_TABINDEX,
	);
}

function createJSXDiagnostic(
	path: CompilerPath,
	node: JSXElement,
	attribute: JSXAttribute,
) {
	return path.addFixableDiagnostic(
		{
			target: attribute,
			fixed: signals.replace({
				...node,
				attributes: node.attributes.filter((attribute) =>
					attribute.type !== "JSXAttribute" ||
					attribute.name.name !== "tabIndex"
				),
			}),
		},
		descriptions.LINT.A11_Y_NO_POSITIVE_TABINDEX,
	);
}

export default createLintVisitor({
	name: "a11y/noPositiveTabindex",
	enter(path) {
		const {node} = path;

		if (isHTMLElement(node) && hasHTMLAttribute(node, "tabindex")) {
			const attribute = getHTMLAttribute(node, "tabindex");
			if (attribute?.value?.type === "HTMLString") {
				const tabIndexValue = attribute.value.value;
				if (Number(tabIndexValue) > 0) {
					createHTMLDiagnostic(path, node, attribute);
				}
			}
		}

		if (isJSXDOMElement(node) && hasJSXAttribute(node, "tabIndex")) {
			const attribute = getJSXAttribute(node, "tabIndex");
			if (attribute?.value?.type === "JSStringLiteral") {
				const tabIndexValue = attribute.value.value;
				if (Number(tabIndexValue) > 0) {
					createJSXDiagnostic(path, node, attribute);
				}
			}

			if (attribute?.value?.type === "JSXExpressionContainer") {
				const expression = attribute.value.expression;
				if (
					expression.type === "JSNumericLiteral" ||
					expression.type === "JSStringLiteral"
				) {
					const tabIndexValue = expression.value;
					if (Number(tabIndexValue) > 0) {
						createJSXDiagnostic(path, node, attribute);
					}
				}
			}
		}

		return signals.retain;
	},
});
