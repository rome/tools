import {ARIARoleDefinition} from "@romejs/compiler/lint/utils/aria/types";

export default function isRoleInteractive(role: ARIARoleDefinition) {
	return role.superClassRole.includes("widget");
}
