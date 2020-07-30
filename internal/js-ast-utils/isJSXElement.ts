import {AnyNode, JSXElement} from "@internal/ast";
import {doesNodeMatchPattern} from "./doesNodeMatchPattern";

export function isJSXElement(node: AnyNode, name: string): node is JSXElement {
	return node.type === "JSXElement" && doesNodeMatchPattern(node.name, name);
}
