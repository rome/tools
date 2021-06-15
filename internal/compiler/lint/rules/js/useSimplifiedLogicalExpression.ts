import {
	AnyJSExpression,
	JSBooleanLiteral,
	JSLogicalExpression,
	JSUnaryExpression,
	jsUnaryExpression,
} from "@internal/ast";
import {CompilerPath, signals} from "@internal/compiler";
import {createLintVisitor} from "@internal/compiler/utils";
import {descriptions} from "@internal/diagnostics";

export default createLintVisitor({
	name: "js/useSimplifiedLogicalExpression",
	enter(path) {
		const {node} = path;

		if (node.type === "JSLogicalExpression") {
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
					descriptions.LINT.JS_USE_SIMPLIFIED_LOGICAL_EXPRESSION,
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

function simplifyAndExpression(
	path: CompilerPath,
	literal: JSBooleanLiteral,
	expression: AnyJSExpression,
) {
	return keepExpressionIfLiteral(path, expression, literal, true);
}

function simplifyOrExpression(
	path: CompilerPath,
	literal: JSBooleanLiteral,
	expression: AnyJSExpression,
) {
	return keepExpressionIfLiteral(path, expression, literal, false);
}

function keepExpressionIfLiteral(
	path: CompilerPath,
	expression: AnyJSExpression,
	literal: JSBooleanLiteral,
	expectedValue: boolean,
) {
	if (literal.value === expectedValue) {
		return path.addFixableDiagnostic(
			{fixed: signals.replace({...expression})},
			descriptions.LINT.JS_USE_SIMPLIFIED_LOGICAL_EXPRESSION,
		);
	}
	return path.addFixableDiagnostic(
		{fixed: signals.replace({...literal})},
		descriptions.LINT.JS_USE_SIMPLIFIED_LOGICAL_EXPRESSION,
	);
}

// https://en.wikipedia.org/wiki/De_Morgan%27s_laws
function couldApplyDeMorgan(
	node: JSLogicalExpression,
): node is DeMorganExpression {
	return (
		node.left.type === "JSUnaryExpression" &&
		node.right.type === "JSUnaryExpression" &&
		node.left.operator === "!" &&
		node.right.operator === "!" &&
		node.left.argument.type !== "JSUnaryExpression" &&
		node.right.argument.type !== "JSUnaryExpression"
	);
}

function simplifyDeMorgan(path: CompilerPath, node: DeMorganExpression) {
	return path.addFixableDiagnostic(
		{
			fixed: signals.replace(
				jsUnaryExpression.create({
					operator: "!",
					argument: {
						...node,
						left: node.left.argument,
						right: node.right.argument,
						operator: node.operator === "||" ? "&&" : "||",
					},
				}),
			),
		},
		descriptions.LINT.JS_USE_SIMPLIFIED_LOGICAL_EXPRESSION,
	);
}

interface NegatedExpression extends JSUnaryExpression {
	operator: "!";
}

interface DeMorganExpression extends JSLogicalExpression {
	left: NegatedExpression;
	right: NegatedExpression;
}
