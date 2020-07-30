import {createVisitor, signals} from "@internal/compiler";
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

export default createVisitor({
	name: "jsx-a11y/noNoninteractiveElementToInteractiveRole",
	enter(path) {
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

		return signals.retain;
	},
});
