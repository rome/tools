import {Path, TransformExitResult} from "@romejs/compiler";
import {descriptions} from "@romejs/diagnostics";
import {getJSXAttribute, hasJSXAttribute} from "@romejs/js-ast-utils";
import getJSXElementName from "@romejs/js-ast-utils/getJSXElementName";
import {
	isElementInteractive,
	isRoleInteractive,
	roles,
} from "@romejs/compiler/lint/rules/ariaHelpers";

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
				const role = roles.get(roleAttribute.value.value);
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
