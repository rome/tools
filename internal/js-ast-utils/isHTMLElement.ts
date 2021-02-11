import {AnyNode, HTMLElement} from "@internal/ast";

export default function isHTMLElement(node: AnyNode): node is HTMLElement {
	return node.type === "HTMLElement";
}
