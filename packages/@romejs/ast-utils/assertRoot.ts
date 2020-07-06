import {TransformExitResult} from "@romejs/compiler";
import {AnyRoot} from "@romejs/ast";
import {isRoot} from "./isRoot";

export function assertRoot(node: TransformExitResult): AnyRoot {
	if (isRoot(node)) {
		return node;
	} else {
		throw new Error("Expected root node");
	}
}
