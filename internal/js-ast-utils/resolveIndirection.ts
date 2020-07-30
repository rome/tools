import {AnyNode} from "@internal/ast";
import {ConstBinding, Scope} from "@internal/compiler";

export function resolveIndirection(
	node: AnyNode,
	scope: Scope,
): {
	node: AnyNode;
	scope: Scope;
} {
	switch (node.type) {
		case "JSReferenceIdentifier": {
			const binding = scope.getBinding(node.name);
			if (binding instanceof ConstBinding && binding.value !== undefined) {
				return resolveIndirection(binding.value, binding.scope);
			}
			break;
		}

		case "JSSequenceExpression":
			return resolveIndirection(
				node.expressions[node.expressions.length - 1],
				scope,
			);

		case "TSAsExpression":
		case "TSTypeAssertion":
		case "TSNonNullExpression":
			return resolveIndirection(node.expression, scope);
	}

	return {node, scope};
}
