import {AnyNode} from "@romefrontend/ast";
import {isCreateElement} from ".";
import {tryStaticEvaluation} from "@romefrontend/js-ast-utils";
import {Scope} from "@romefrontend/compiler";

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
