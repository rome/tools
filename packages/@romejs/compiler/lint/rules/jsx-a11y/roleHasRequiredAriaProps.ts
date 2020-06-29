import {Path, TransformExitResult} from "@romejs/compiler";
import {descriptions} from "@romejs/diagnostics";
import {getJSXAttribute, hasJSXAttribute} from "@romejs/js-ast-utils";
import {ariaRolesMap} from "../../utils/aria";

export default {
	name: "jsxA11YRoleHasRequiredAriaProps",
	enter(path: Path): TransformExitResult {
		const {node} = path;

		if (node.type === "JSXElement" && hasJSXAttribute(node, "role")) {
			const attr = getJSXAttribute(node, "role");
			if (attr && attr.value && attr.value.type === "JSStringLiteral") {
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

		return node;
	},
};
