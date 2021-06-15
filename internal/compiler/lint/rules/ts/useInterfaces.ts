import {Scope, createLintVisitor, signals} from "@internal/compiler";
import {
	AnyTSTypeElement,
	TSExpressionWithTypeArguments,
	TSTypeAlias,
} from "@internal/ast";
import {getTSQualifiedBaseFromEntityName} from "@internal/js-ast-utils";
import {TypeBinding} from "@internal/compiler/scope/bindings";

function extractObjects(
	typeAlias: TSTypeAlias,
	scope: Scope,
):
	| undefined
	| {
			extends: TSExpressionWithTypeArguments[];
			members: AnyTSTypeElement[];
		} {
	const {right} = typeAlias;

	let types;
	if (right.type === "TSObjectTypeAnnotation") {
		types = [right];
	} else if (right.type === "TSIntersectionTypeAnnotation") {
		types = right.types;
	} else {
		return undefined;
	}

	const _extends: TSExpressionWithTypeArguments[] = [];
	let members: AnyTSTypeElement[] = [];

	for (const node of types) {
		if (node.type === "TSObjectTypeAnnotation") {
			members = members.concat(node.members);
		} else if (node.type === "TSTypeReference") {
			const base = getTSQualifiedBaseFromEntityName(node.typeName);
			const binding = scope.getBinding(base.name);

			if (
				binding === undefined ||
				(binding instanceof TypeBinding && binding.typeKind === "parameter")
			) {
				// `extends` can only access a static type
				return undefined;
			}

			_extends.push({
				type: "TSExpressionWithTypeArguments",
				expression: node.typeName,
				typeParameters: node.typeParameters,
			});
		} else {
			// No idea what this is or how to include it
			return undefined;
		}
	}

	return {extends: _extends, members};
}

export default createLintVisitor({
	name: "ts/useInterfaces",
	enter(path) {
		const {node} = path;

		if (node.type === "TSTypeAlias") {
			const extracted = extractObjects(node, path.scope);

			if (extracted !== undefined) {
				/*return path.context.addFixableDiagnostic(
					{
						old: node,
						suggestions: [{
							fixed: {
								type: "TSInterfaceDeclaration",
								loc: node.loc,
								id: node.id,
								typeParameters: node.typeParameters,
								declare: node.declare,
								extends: extracted.extends,
								body: {
									type: "TSInterfaceBody",
									body: extracted.members,
								},
							},
						}]
					},
					descriptions.LINT.TS_PREFER_INTERFACES,
				);*/
			}
		}

		return signals.retain;
	},
});
