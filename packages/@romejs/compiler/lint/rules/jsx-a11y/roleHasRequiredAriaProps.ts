import {Path, TransformExitResult} from "@romejs/compiler";
import {descriptions} from "@romejs/diagnostics";
import {getJSXAttribute, hasJSXAttribute, isJSXElement} from "@romejs/js-ast-utils";
import {roles} from "@romejs/compiler/lint/rules/aria-helpers";

export default {
 name: "roleHasRequiredAriaProps",
 enter(path: Path): TransformExitResult {
	 const {node} = path;

	 if (isJSXElement(node) && hasJSXAttribute(node, 'role')) {
	 	const attr = getJSXAttribute(node, 'role');
	 	if (attr && attr.value && attr.value.type === "JSStringLiteral")  {
			const role = roles.get(attr.value.value);
			if (role) {
				const hasAllProps = role.requiredProps.every(prop => {
					return hasJSXAttribute(node, prop);
				})

				if (!hasAllProps) {
					path.context.addNodeDiagnostic(
						node,
						descriptions.LINT.JSX_A11Y_ROLE_HAS_REQUIRED_ARIA_PROPS(attr.value.value, role.requiredProps),
					);
				}
			}
		}
	 }

	 return node;
 },
};
