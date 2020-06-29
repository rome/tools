import {
	ariaRolesMap,
	isRoleInteractive,
} from "@romejs/compiler/lint/utils/aria/index";
import {ARIARoleDefinition} from "@romejs/compiler/lint/utils/aria/types";

export default function isElementInteractive(elementName: string) {
	let role: ARIARoleDefinition | undefined;
	for (const [, roleInfo] of ariaRolesMap) {
		if (roleInfo.baseConcepts) {
			const elementMatched = roleInfo.baseConcepts.some(({concept}) =>
				concept.name === elementName
			);
			if (elementMatched) {
				role = roleInfo;
				break;
			}
		}
	}

	if (role) {
		return isRoleInteractive(role);
	}
	return false;
}
