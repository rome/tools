import {AnyRoot} from "@romejs/ast/unions";
import {TransformExitResult} from "@romejs/compiler";

export function isRoot(node: TransformExitResult): node is AnyRoot {
	return (
		!Array.isArray(node) && typeof node !== "symbol" && node.type === "JSRoot"
	);
}
