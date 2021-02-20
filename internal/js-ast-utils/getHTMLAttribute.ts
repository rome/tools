import {HTMLAttribute, HTMLElement} from "@internal/ast";

export default function getHTMLAttribute(
	node: HTMLElement,
	attributeName: string,
	allowEmpty = false,
): HTMLAttribute | undefined {
	for (const attr of node.attributes) {
		if (attr.type === "HTMLAttribute" && attr.name.name === attributeName) {
			const {value} = attr;

			if (value !== undefined && !allowEmpty && value.value === "") {
				return undefined;
			}

			return attr;
		}
	}
	return undefined;
}
