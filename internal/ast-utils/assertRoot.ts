import {AnyNode, AnyRoot} from "@internal/ast";
import {isRoot} from "./isRoot";

export function assertRoot(node: AnyNode): AnyRoot | never {
	if (isRoot(node)) {
		return node;
	}
	throw new Error(`Expected root node but got ${node.type}`);
}
