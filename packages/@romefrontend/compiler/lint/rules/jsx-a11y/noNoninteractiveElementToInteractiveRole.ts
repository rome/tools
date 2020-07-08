import {Path, TransformExitResult} from "@romefrontend/compiler";
import {descriptions} from "@romefrontend/diagnostics";
import {
	getJSXAttribute,
	getJSXElementName,
	hasJSXAttribute,
} from "@romefrontend/js-ast-utils";
import {
	ariaRolesMap,
	isElementInteractive,
	isRoleInteractive,
} from "@romefrontend/compiler/lint/utils/aria";

export default {
	name: "jsxA11YNoNoninteractiveElementToInteractiveRole",
	enter(path: Path): TransformExitResult {
		const {node} = path;

		if (node.type === "JSXElement" && hasJSXAttribute(node, "role")) {
			const name = getJSXElementName(node);
			const roleAttribute = getJSXAttribute(node, "role");
			if (
				roleAttribute &&
				roleAttribute.value &&
				roleAttribute.value.type === "JSStringLiteral"
			) {
				const role = ariaRolesMap.get(roleAttribute.value.value);
				if (role) {
					if (!isElementInteractive(name) && isRoleInteractive(role)) {
						path.context.addNodeDiagnostic(
							roleAttribute,
							descriptions.LINT.JSX_A11Y_NO_NONINTERACTIVE_ELEMENT_TO_INTERACTIVE_ROLE(
								name,
							),
						);
					}
				}
			}
		}

		return node;
	},
};
