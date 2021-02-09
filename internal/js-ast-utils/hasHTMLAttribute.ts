import {HTMLElement} from "@internal/ast";
import getHTMLAttribute from "@internal/js-ast-utils/getHTMLAttribute";

export default function hasHTMLAttribute(
	node: HTMLElement,
	attributeName: string,
): boolean {
	return getHTMLAttribute(node, attributeName, true) !== undefined;
}
