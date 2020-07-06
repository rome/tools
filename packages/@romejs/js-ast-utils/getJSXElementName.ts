import {JSXElement} from "@romejs/ast";

export function getJSXElementName(node: JSXElement): string {
	if (node.name.type === "JSXIdentifier") {
		return node.name.name;
	}
	if (node.name.type === "JSXReferenceIdentifier") {
		return node.name.name;
	}
	if (node.name.type === "JSXNamespacedName") {
		return node.name.name.name;
	}

	return "";
}
