import {AnyNode, JSXElement} from "@romejs/ast";
import doesNodeMatchPattern from "./doesNodeMatchPattern";

export default function isJSXElement(
	node: AnyNode,
	name: string,
): node is JSXElement {
	return node.type === "JSXElement" && doesNodeMatchPattern(node.name, name);
}
