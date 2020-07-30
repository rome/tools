import {AnyNode, AnyRoot} from "@internal/ast";

export function isRoot(node: AnyNode): node is AnyRoot {
	return (
		node.type === "JSRoot" ||
		node.type === "CSSRoot" ||
		node.type === "CommitRoot" ||
		node.type === "MarkdownRoot" ||
		node.type === "HTMLRoot"
	);
}
