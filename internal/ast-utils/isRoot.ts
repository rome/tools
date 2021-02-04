import {AnyNode, AnyRoot} from "@internal/ast";

const roots = new Set([
	"JSRoot",
	"CSSRoot",
	"CommitRoot",
	"MarkdownRoot",
	"HTMLRoot",
]);

export function isRoot(node: AnyNode): node is AnyRoot {
	return roots.has(node.type);
}
