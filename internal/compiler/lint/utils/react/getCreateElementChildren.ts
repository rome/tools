import {Scope} from "@internal/compiler";
import {AnyNode} from "@internal/ast";
import getCreateElementProp from "./getCreateElementProp";
import isCreateElement from "./isCreateElement";

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
