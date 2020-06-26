import {AnyNode, JSObjectProperty} from "@romejs/ast";
import {isCreateElement} from ".";
import {Scope} from "@romejs/compiler";

export default function getCreateElementProp(
	node: AnyNode,
	scope: Scope,
	propName: string,
): JSObjectProperty | undefined {
	if (
		!isCreateElement(node, scope) ||
		!node.arguments[1] ||
		node.arguments[1].type !== "JSObjectExpression"
	) {
		return;
	}

	return (node.arguments[1].properties.find((prop) => {
		return (
			prop.type === "JSObjectProperty" &&
			prop.key.value.type === "JSIdentifier" &&
			prop.key.value.name === propName
		);
	}) as JSObjectProperty | undefined);
}
