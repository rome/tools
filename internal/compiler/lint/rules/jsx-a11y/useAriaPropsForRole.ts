import {createVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";
import {getJSXAttribute, hasJSXAttribute} from "@internal/js-ast-utils";
import {ariaRolesMap} from "@internal/compiler/lint/utils/aria";
import {isJSXDOMElement} from "@internal/js-ast-utils/isJSXDOMElement";

export default createVisitor({
	name: "jsx-a11y/useAriaPropsForRole",
	enter(path) {
		const {node} = path;

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
							descriptions.LINT.JSX_A11Y_ROLE_HAS_REQUIRED_ARIA_PROPS(
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
