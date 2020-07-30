import {Scope} from "@internal/compiler";
import {AnyNode} from "@internal/ast";
import {getCreateElementProp, isCreateElement} from ".";

export default function getCreateElementChildren(
	node: AnyNode,
	scope: Scope,
): AnyNode | Array<AnyNode> | undefined {
	if (!isCreateElement(node, scope)) {
		return;
	}
	if (node.arguments.length > 2) {
		return node.arguments.slice(2);
	}
	return getCreateElementProp(node, scope, "children");
}
