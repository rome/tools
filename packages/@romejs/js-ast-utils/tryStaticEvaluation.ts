import {
	AnyJSExpression,
	JSBinaryExpression,
	JSUnaryExpression,
} from "@romejs/ast";

function evalConstant(expr: AnyJSExpression) {
	switch (expr.type) {
		case "JSStringLiteral":
			return expr.value;
		case "JSUnaryExpression":
			return evalUnaryExpression(expr);
		case "JSBinaryExpression":
			return evalBinaryExpression(expr);
		case "JSNumericLiteral":
			return expr.value;
		case "JSTemplateLiteral": {
			if (expr.quasis.length === 1) {
				return expr.quasis[0].cooked;
			}
			return undefined;
		}
		default:
			return undefined;
	}
}

function evalUnaryExpression(
	expr: JSUnaryExpression,
): number | string | undefined {
	const value = evalConstant(expr.argument);
	if (value === undefined) {
		return;
	}

	switch (expr.operator) {
		case "+":
			return value;
		case "-":
			return -value;
		case "~":
			return ~value;
		default:
			return undefined;
	}
}

function evalBinaryExpression(expr: JSBinaryExpression): number | undefined {
	const left = Number(evalConstant(expr.left));
	if (left === undefined) {
		return;
	}
	const right = Number(evalConstant(expr.right));
	if (right === undefined) {
		return;
	}

	switch (expr.operator) {
		case "|":
			return left | right;
		case "&":
			return left & right;
		case ">>":
			return left >> right;
		case ">>>":
			return left >>> right;
		case "<<":
			return left << right;
		case "^":
			return left ^ right;
		case "*":
			return left * right;
		case "/":
			return left / right;
		case "+":
			return left + right;
		case "-":
			return left - right;
		case "%":
			return left % right;
		default:
			return undefined;
	}
}

export default function tryStaticEvaluation(
	expr: AnyJSExpression,
): {
	value?: string | number;
	bailed?: boolean;
} {
	const value = evalConstant(expr);
	return {
		value,
		bailed: !value,
	};
}
