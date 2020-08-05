import {AnyNode} from "@internal/ast";
import isCreateElement from "./isCreateElement";
import {tryStaticEvaluation} from "@internal/js-ast-utils";
import {Scope} from "@internal/compiler";

export default function getCreateElementType(
	node: AnyNode,
	scope: Scope,
): string | undefined {
	if (!isCreateElement(node, scope) || !node.arguments[0]) {
		return;
	}
	const {bailed, value} = tryStaticEvaluation(node.arguments[0], scope);
	return !bailed && typeof value === "string" ? value : undefined;
}
