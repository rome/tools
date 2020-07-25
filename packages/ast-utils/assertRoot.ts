import {TransformExitResult} from "@romefrontend/compiler";
import {AnyRoot} from "@romefrontend/ast";
import {isRoot} from "./isRoot";

export function assertRoot(node: TransformExitResult): AnyRoot {
	if (isRoot(node)) {
		return node;
	} else {
		throw new Error("Expected root node");
	}
}
