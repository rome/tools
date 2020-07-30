import {AnyNode, JSCallExpression} from "@internal/ast";
import {isIdentifierish, resolveIndirection} from "@internal/js-ast-utils";
import {ImportBinding, Scope} from "@internal/compiler";

export default function isCreateElement(
	node: AnyNode,
	scope: Scope,
): node is JSCallExpression {
	let isCreateElement = false;

	if (node.type !== "JSCallExpression") {
		return isCreateElement;
	}

	if (isIdentifierish(node.callee)) {
		const {node: callee} = resolveIndirection(node.callee, scope);
		if (!isIdentifierish(callee)) {
			return isCreateElement;
		}
		const reference = scope.getBinding(callee.name);
		isCreateElement =
			(!reference && callee.name === "createElement") ||
			(reference instanceof ImportBinding &&
			reference.meta.source === "react" &&
			reference.meta.type === "name" &&
			reference.meta.imported === "createElement");
	}

	if (node.callee.type === "JSMemberExpression") {
		if (!isIdentifierish(node.callee.object)) {
			return isCreateElement;
		}
		const {node: object} = resolveIndirection(node.callee.object, scope);
		if (!isIdentifierish(object)) {
			return isCreateElement;
		}
		const reference = scope.getBinding(object.name);
		isCreateElement =
			node.callee.property.type === "JSStaticMemberProperty" &&
			node.callee.property.value.type === "JSIdentifier" &&
			node.callee.property.value.name === "createElement" &&
			((reference instanceof ImportBinding && reference.meta.source === "react") ||
			(!reference && object.name === "React"));
	}

	return isCreateElement;
}
