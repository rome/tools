import {AnyNode, AnyRoot} from "@internal/ast";

const roots: Set<string> = new Set([
	"JSRoot",
	"CSSRoot",
	"MarkdownRoot",
	"HTMLRoot",
]);

export function isRoot(node: AnyNode): node is AnyRoot {
	return roots.has(node.type);
}
