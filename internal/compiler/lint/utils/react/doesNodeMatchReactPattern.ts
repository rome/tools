import {ImportBinding, Scope} from "@internal/compiler";
import {AnyNode} from "@internal/ast";
import {isIdentifierish, resolveIndirection} from "@internal/js-ast-utils";

export interface MatchConfig {
	packageName: string;
	importName: string;
}

export default function doesNodeMatchReactPattern(
	node: AnyNode,
	scope: Scope,
	pattern: string,
	config: MatchConfig = {
		packageName: "react",
		importName: "React",
	},
): boolean {
	if (isIdentifierish(node)) {
		({node, scope} = resolveIndirection(node, scope));
		if (!isIdentifierish(node)) {
			return false;
		}

		const reference = scope.getBinding(node.name);
		return (
			(!reference && node.name === pattern) ||
			(reference instanceof ImportBinding &&
			reference.meta.source === config.packageName &&
			reference.meta.type === "name" &&
			reference.meta.imported === pattern)
		);
	}

	if (node.type === "JSMemberExpression" || node.type === "JSXMemberExpression") {
		const member = pattern.replace(new RegExp(`^${config.importName}\.`), "");
		const {node: objectNode, scope: objectScope} = resolveIndirection(
			node.object,
			scope,
		);
		if (!isIdentifierish(objectNode)) {
			return false;
		}

		const reference = objectScope.getBinding(objectNode.name);
		let identifier;
		if (
			node.property.type === "JSStaticMemberProperty" &&
			node.property.value.type === "JSIdentifier"
		) {
			identifier = node.property.value;
		} else if (node.property.type === "JSXIdentifier") {
			identifier = node.property;
		}
		if (!identifier) {
			return false;
		}

		return (
			(!reference && `${objectNode.name}.${identifier.name}` === pattern) ||
			(reference instanceof ImportBinding &&
			reference.meta.source === config.packageName &&
			identifier.name === member)
		);
	}
	return false;
}
