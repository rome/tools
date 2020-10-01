import {
	AnyJSExpression,
	JSBooleanLiteral,
	JSLogicalExpression,
	JSUnaryExpression,
} from "@internal/ast";
import { Path, signals } from "@internal/compiler";
import { createVisitor } from "@internal/compiler/utils";
import { descriptions } from "@internal/diagnostics";

export default createVisitor({
	name: "js/useSimplifiedBooleanExpression",
	enter(path) {
		const { node } = path;

		if (node.type === "JSBinaryExpression") {
			if (
				node.operator === "===" ||
				node.operator === "==" ||
				node.operator === "!==" ||
				node.operator === "!="
			) {
				// if the operator is "not equal" (exclusive) or one of the operands is "false"
				// we consider the expression to be "negated"
				let negated = node.operator === "!==" || node.operator === "!=";

				if (node.left.type === "JSBooleanLiteral") {
					return simplifyBinaryExpressionDiagnostic(
						path,
						node.left,
						node.right,
						negated
					);
				}
				if (node.right.type === "JSBooleanLiteral") {
					return simplifyBinaryExpressionDiagnostic(
						path,
						node.right,
						node.left,
						negated
					);
				}
			}
		} else if (
			node.type === "JSUnaryExpression" &&
			node.operator === "!" &&
			node.argument.type === "JSUnaryExpression" &&
			node.argument.operator === "!"
		) {
			return path.addFixableDiagnostic(
				{
					fixed: signals.replace({ ...node.argument.argument }),
				},
				descriptions.LINT.JS_USE_SIMPLIFIED_BOOLEAN_EXPRESSION
			);
		} else if (node.type === "JSLogicalExpression") {
			if (node.operator === "&&") {
				if (node.left.type === "JSBooleanLiteral") {
					return simplifyAndExpression(path, node.left, node.right);
				}
				if (node.right.type === "JSBooleanLiteral") {
					return simplifyAndExpression(path, node.right, node.left);
				}

				if (couldApplyDeMorgan(node)) {
					return simplifyDeMorgan(path, node);
				}
			} else if (node.operator === "||") {
				if (node.left.type === "JSBooleanLiteral") {
					return simplifyOrExpression(path, node.left, node.right);
				}
				if (node.right.type === "JSBooleanLiteral") {
					return simplifyOrExpression(path, node.right, node.left);
				}

				if (couldApplyDeMorgan(node)) {
					return simplifyDeMorgan(path, node);
				}
			} else if (node.operator === "??" && node.left.type === "JSNullLiteral") {
				return path.addFixableDiagnostic(
					{
						fixed: signals.replace({
							...node.right,
						}),
					},
					descriptions.LINT.JS_USE_SIMPLIFIED_BOOLEAN_EXPRESSION
				);
			}
		}
		/* TODO: simplify the following case when the AST will support it
        if (boolExp) {
                return true;
            } 
            return false;
        */

		return signals.retain;
	},
});

function simplifyBinaryExpressionDiagnostic(
	path: Path,
	toRemove: JSBooleanLiteral,
	expression: AnyJSExpression,
	negated: boolean
) {
	const shouldNegate = xor(negated, !toRemove.value);
	return path.addFixableDiagnostic(
		{
			fixed: signals.replace(createSimpleExpression(expression, shouldNegate)),
		},
		descriptions.LINT.JS_USE_SIMPLIFIED_BOOLEAN_EXPRESSION
	);
}

function createSimpleExpression(
	expression: AnyJSExpression,
	shouldNegate: boolean
): AnyJSExpression {
	return shouldNegate
		? {
				type: "JSUnaryExpression",
				operator: "!",
				argument: expression,
		  }
		: {
				...expression,
		  };
}

function simplifyAndExpression(
	path: Path,
	literal: JSBooleanLiteral,
	expression: AnyJSExpression
) {
	return keepExpressionIfLiteral(path, expression, literal, true);
}

function simplifyOrExpression(
	path: Path,
	literal: JSBooleanLiteral,
	expression: AnyJSExpression
) {
	return keepExpressionIfLiteral(path, expression, literal, false);
}

function keepExpressionIfLiteral(
	path: Path,
	expression: AnyJSExpression,
	literal: JSBooleanLiteral,
	expectedValue: boolean
) {
	if (literal.value === expectedValue) {
		return path.addFixableDiagnostic(
			{ fixed: signals.replace({ ...expression }) },
			descriptions.LINT.JS_USE_SIMPLIFIED_BOOLEAN_EXPRESSION
		);
	}
	return path.addFixableDiagnostic(
		{ fixed: signals.replace({ ...literal }) },
		descriptions.LINT.JS_USE_SIMPLIFIED_BOOLEAN_EXPRESSION
	);
}

// https://en.wikipedia.org/wiki/De_Morgan%27s_laws
function couldApplyDeMorgan(
	node: JSLogicalExpression
): node is DeMorganExpression {
	return (
		node.left.type === "JSUnaryExpression" &&
		node.right.type === "JSUnaryExpression" &&
		node.left.operator === "!" &&
		node.right.operator === "!"
	);
}

function simplifyDeMorgan(path: Path, node: DeMorganExpression) {
	return path.addFixableDiagnostic(
		{
			fixed: signals.replace({
				type: "JSUnaryExpression",
				operator: "!",
				argument: {
					...node,
					left: node.left.argument,
					right: node.right.argument,
					operator: node.operator === "||" ? "&&" : "||",
				},
			}),
		},
		descriptions.LINT.JS_USE_SIMPLIFIED_BOOLEAN_EXPRESSION
	);
}

interface NegatedExpression extends JSUnaryExpression {
	operator: "!";
}

interface DeMorganExpression extends JSLogicalExpression {
	left: NegatedExpression;
	right: NegatedExpression;
}

function xor(x: boolean, y: boolean) {
	return x ? !y : y;
}
