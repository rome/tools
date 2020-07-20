import {AnyRoot} from "@romefrontend/ast/unions";
import {TransformExitResult} from "@romefrontend/compiler";

export function isRoot(node: TransformExitResult): node is AnyRoot {
	return (
		!Array.isArray(node) &&
		typeof node !== "symbol" &&
		(node.type === "JSRoot" ||
		node.type === "CSSStylesheet" ||
		node.type === "MarkdownRoot" ||
		node.type === "HTMLRoot")
	);
}
