import {AnyNode, JSXElement} from "@romejs/js-ast";
import doesNodeMatchPattern from "./doesNodeMatchPattern";

export default function isJSXElement(
	node: AnyNode,
	name?: string,
): node is JSXElement {
	return (
		node.type === "JSXElement" &&
		(name ? doesNodeMatchPattern(node.name, name) : true)
	);
}
