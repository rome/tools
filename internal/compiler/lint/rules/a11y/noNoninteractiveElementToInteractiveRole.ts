import {createLintVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";
import {
	getJSXAttribute,
	getJSXElementName,
	hasJSXAttribute,
} from "@internal/js-ast-utils";
import {
	ariaRolesMap,
	isElementInteractive,
	isRoleInteractive,
} from "@internal/compiler/lint/utils/aria";
import {isJSXDOMElement} from "@internal/js-ast-utils/isJSXDOMElement";
import isHTMLElement from "@internal/js-ast-utils/isHTMLElement";
import htmlAttributeHasValue from "@internal/js-ast-utils/htmlAttributeHasValue";
import getHTMLAttribute from "@internal/js-ast-utils/getHTMLAttribute";

export default createLintVisitor({
	name: "a11y/noNoninteractiveElementToInteractiveRole",
	enter(path) {
		const {node} = path;
		if (isHTMLElement(node) && htmlAttributeHasValue(node, "role")) {
			const name = node.name.name;
			const roleAttribute = getHTMLAttribute(node, "role");
			if (roleAttribute?.value) {
				const role = ariaRolesMap.get(roleAttribute.value.value);
				if (role) {
					if (!isElementInteractive(name) && isRoleInteractive(role)) {
						path.context.addNodeDiagnostic(
							roleAttribute,
							descriptions.LINT.A11_Y_NO_NONINTERACTIVE_ELEMENT_TO_INTERACTIVE_ROLE(
								name,
							),
						);
					}
				}
			}
		} else if (isJSXDOMElement(node) && hasJSXAttribute(node, "role")) {
			const name = getJSXElementName(node);
			const roleAttribute = getJSXAttribute(node, "role");
			if (roleAttribute?.value?.type === "JSStringLiteral") {
				const role = ariaRolesMap.get(roleAttribute.value.value);
				if (role) {
					if (!isElementInteractive(name) && isRoleInteractive(role)) {
						path.context.addNodeDiagnostic(
							roleAttribute,
							descriptions.LINT.A11_Y_NO_NONINTERACTIVE_ELEMENT_TO_INTERACTIVE_ROLE(
								name,
							),
						);
					}
				}
			}
		}

		return signals.retain;
	},
});
