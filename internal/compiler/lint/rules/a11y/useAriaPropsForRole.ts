import {createLintVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";
import {getJSXAttribute, hasJSXAttribute} from "@internal/js-ast-utils";
import {ariaRolesMap} from "@internal/compiler/lint/utils/aria";
import {isJSXDOMElement} from "@internal/js-ast-utils/isJSXDOMElement";
import isHTMLElement from "@internal/js-ast-utils/isHTMLElement";
import hasHTMLAttribute from "@internal/js-ast-utils/hasHTMLAttribute";
import getHTMLAttribute from "@internal/js-ast-utils/getHTMLAttribute";

export default createLintVisitor({
	name: "a11y/useAriaPropsForRole",
	enter(path) {
		const {node} = path;

		if (isHTMLElement(node) && hasHTMLAttribute(node, "role")) {
			const attr = getHTMLAttribute(node, "role");
			if (attr?.value?.type === "HTMLString") {
				const role = ariaRolesMap.get(attr.value.value);
				if (role) {
					const hasAllProps = role.requiredProps.every((prop) => {
						return hasHTMLAttribute(node, prop);
					});

					if (!hasAllProps) {
						const missingProps = role.requiredProps.map((prop) => {
							const attr = getHTMLAttribute(node, prop);
							if (!attr) {
								return prop;
							}
							return "";
						}).filter(Boolean);
						path.context.addNodeDiagnostic(
							node,
							descriptions.LINT.A11Y_ROLE_HAS_REQUIRED_ARIA_PROPS(
								attr.value.value,
								missingProps,
							),
						);
					}
				}
			}
		}

		if (isJSXDOMElement(node) && hasJSXAttribute(node, "role")) {
			const attr = getJSXAttribute(node, "role");
			if (attr?.value?.type === "JSStringLiteral") {
				const role = ariaRolesMap.get(attr.value.value);
				if (role) {
					const hasAllProps = role.requiredProps.every((prop) => {
						return hasJSXAttribute(node, prop);
					});

					if (!hasAllProps) {
						const missingProps = role.requiredProps.map((prop) => {
							const attr = getJSXAttribute(node, prop);
							if (!attr) {
								return prop;
							}
							return "";
						}).filter(Boolean);
						path.context.addNodeDiagnostic(
							node,
							descriptions.LINT.A11Y_ROLE_HAS_REQUIRED_ARIA_PROPS(
								attr.value.value,
								missingProps,
							),
						);
					}
				}
			}
		}

		return signals.retain;
	},
});
