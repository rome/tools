import {HTMLElement} from "@internal/ast";
import getHTMLAttribute from "@internal/js-ast-utils/getHTMLAttribute";

export default function htmlAttributeHasValue(
	node: HTMLElement,
	attributeName: string,
): boolean {
	const attr = getHTMLAttribute(node, attributeName);
	if (attr) {
		return (
			attr?.value?.type === "HTMLString" &&
			attr?.value?.value !== "" &&
			attr?.value?.value !== undefined
		);
	}
	return false;
}
