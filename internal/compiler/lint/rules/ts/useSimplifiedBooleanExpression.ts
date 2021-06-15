import {
	AnyJSExpression,
	AnyNode,
	JSBooleanLiteral,
	jsUnaryExpression,
} from "@internal/ast";
import {
	CompilerPath,
	Scope,
	createLintVisitor,
	signals,
} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";

import {resolveIndirection} from "@internal/js-ast-utils";

export default createLintVisitor({
	name: "ts/useSimplifiedBooleanExpression",
	enter(path) {
		const {node, scope} = path;

		if (node.type === "JSBinaryExpression") {
			if (node.operator === "===" || node.operator === "!==") {
				// if the operator is "not equal" (exclusive) or one of the operands is "false"
				// we consider the expression to be "negated"
				let negated = node.operator === "!==";

				if (node.left.type === "JSBooleanLiteral") {
					const {
						node: resolvedNode,
						scope: resolvedScope,
					} = resolveIndirection(node.right, scope);

					if (isBooleanType(resolvedNode, resolvedScope)) {
						return simplifyBinaryExpression(
							path,
							node.left,
							node.right,
							negated,
						);
					}
				}
				if (node.right.type === "JSBooleanLiteral") {
					const {
						node: resolvedNode,
						scope: resolvedScope,
					} = resolveIndirection(node.left, scope);

					if (isBooleanType(resolvedNode, resolvedScope)) {
						return simplifyBinaryExpression(
							path,
							node.right,
							node.left,
							negated,
						);
					}
				}
			}
		} else if (
			node.type === "JSUnaryExpression" &&
			node.operator === "!" &&
			node.argument.type === "JSUnaryExpression" &&
			node.argument.operator === "!"
		) {
			const realArgument = node.argument.argument;
			const {node: resolvedNode, scope: resolvedScope} = resolveIndirection(
				realArgument,
				scope,
			);
			if (isBooleanType(resolvedNode, resolvedScope)) {
				return path.addFixableDiagnostic(
					{
						fixed: signals.replace({...node.argument.argument}),
					},
					descriptions.LINT.TS_USE_SIMPLIFIED_BOOLEAN_EXPRESSION,
				);
			}
		}

		return signals.retain;
	},
});

function simplifyBinaryExpression(
	path: CompilerPath,
	toRemove: JSBooleanLiteral,
	expression: AnyJSExpression,
	negated: boolean,
) {
	const shouldNegate = xor(negated, !toRemove.value);
	return path.addFixableDiagnostic(
		{
			fixed: signals.replace(createSimpleExpression(expression, shouldNegate)),
		},
		descriptions.LINT.TS_USE_SIMPLIFIED_BOOLEAN_EXPRESSION,
	);
}

function createSimpleExpression(
	expression: AnyJSExpression,
	shouldNegate: boolean,
): AnyJSExpression {
	return shouldNegate
		? jsUnaryExpression.create({
				operator: "!",
				argument: expression,
			})
		: {
				...expression,
			};
}

function isBooleanType(node: AnyNode, scope: Scope): boolean {
	if (node.type === "JSBooleanLiteral") {
		return true;
	}
	if (node.type === "JSLogicalExpression") {
		return true;
	}

	if (node.type === "JSReferenceIdentifier") {
		const binding = scope.getBinding(node.name);
		return (
			binding?.node.type === "JSBindingIdentifier" &&
			binding.node.meta?.optional !== true &&
			(binding.node.meta?.typeAnnotation?.type ===
			"TSBooleanKeywordTypeAnnotation" ||
			binding.node.meta?.typeAnnotation?.type ===
			"TSBooleanLiteralTypeAnnotation")
		);
	}

	return false;
}

function xor(x: boolean, y: boolean) {
	return x ? !y : y;
}
