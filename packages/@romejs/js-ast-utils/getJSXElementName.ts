import {JSXElement} from "@romejs/ast";

export default function getJSXElementName(node: JSXElement): string {
	if (node.name.type === "JSXIdentifier") {
		return node.name.name;
	}
	if (node.name.type === "JSXReferenceIdentifier") {
		return node.name.name;
	}
	if (node.name.type === "JSXMemberExpression") {
		// TODO need to understand what to do here, what's the case
		return "";
	}

	if (node.name.type === "JSXNamespacedName") {
		return node.name.name.name;
	}

	return "";
}
