import {AnyNode} from "@romejs/ast";
import {AnyRoot} from "@romejs/ast/unions";

export function isRoot(node: AnyNode): node is AnyRoot {
	return node.type === "JSRoot";
}
