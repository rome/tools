import {ARIARoleDefinition} from "@internal/compiler/lint/utils/aria";

export default function isRoleInteractive(role: ARIARoleDefinition) {
	return role.superClassRole.includes("widget");
}
