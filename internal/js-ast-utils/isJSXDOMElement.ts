import {AnyNode, JSXElement} from "@internal/ast";

export function isJSXDOMElement(node: AnyNode): node is JSXElement {
	return node.type === "JSXElement" && node.name.type === "JSXIdentifier";
}
