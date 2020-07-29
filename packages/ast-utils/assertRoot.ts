import {AnyNode, AnyRoot} from "@romefrontend/ast";
import {isRoot} from "./isRoot";

export function assertRoot(node: AnyNode): AnyRoot {
	if (isRoot(node)) {
		return node;
	} else {
		throw new Error(`Expected root node but got ${node.type}`);
	}
}
