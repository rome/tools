import {
	AnyNode,
	JSBinaryExpression,
	JSTemplateLiteral,
	JSUnaryExpression,
} from "@romejs/ast";
import {ConstBinding, Scope} from "@romejs/compiler";

export type EvalResult = {
	value: undefined | null | string | number | boolean;
	bailed: boolean;
};

export type EvalOptions = {};

const BAILED: EvalResult = {
	bailed: true,
	value: undefined,
};

function createResult(value: EvalResult["value"]): EvalResult {
	return {
		bailed: false,
		value,
	};
}

function evalUnaryExpression(
	expr: JSUnaryExpression,
	scope: Scope,
	opts: EvalOptions,
): EvalResult {
	const res = tryStaticEvaluation(expr.argument, scope, opts);

	if (!res.bailed) {
		// We do not care about TS protections
		// rome-ignore lint/js/noExplicitAny
		const value = (res.value as any);

		switch (expr.operator) {
			case "+":
				return createResult(+value);

			case "-":
				return createResult(-value);

			case "~":
				return createResult(~value);
		}
	}

	return BAILED;
}

function evalBinaryExpression(
	expr: JSBinaryExpression,
	scope: Scope,
	opts: EvalOptions,
): EvalResult {
	const left = tryStaticEvaluation(expr.left, scope, opts);
	if (left.bailed) {
		return BAILED;
	}

	const right = tryStaticEvaluation(expr.right, scope, opts);
	if (right.bailed) {
		return BAILED;
	}

	// We do not care about TS protections
	// rome-ignore lint/js/noExplicitAny
	const l = (left.value as any);
	// rome-ignore lint/js/noExplicitAny
	const r = (right.value as any);

	switch (expr.operator) {
		case "|":
			return createResult(l | r);

		case "&":
			return createResult(l & r);

		case ">>":
			return createResult(l >> r);

		case ">>>":
			return createResult(l >>> r);

		case "<<":
			return createResult(l << r);

		case "^":
			return createResult(l ^ r);

		case "*":
			return createResult(l * r);

		case "/":
			return createResult(l / r);

		case "+":
			return createResult(l + r);

		case "-":
			return createResult(l - r);

		case "%":
			return createResult(l % r);

		default:
			return BAILED;
	}
}

function evalTemplateLiteral(
	expr: JSTemplateLiteral,
	scope: Scope,
	opts: EvalOptions,
): EvalResult {
	const {expressions, quasis} = expr;

	let str = "";
	let bailed = false;
	let index = 0;

	for (const elem of quasis) {
		str += elem.cooked;

		if (index < expressions.length) {
			const res = tryStaticEvaluation(expressions[index++], scope, opts);

			if (res.bailed) {
				bailed = true;
				break;
			}

			str += res.value;
		}
	}

	if (bailed) {
		return BAILED;
	} else {
		return createResult(str);
	}
}

const cache: WeakMap<AnyNode, EvalResult> = new WeakMap();

export default function tryStaticEvaluation(
	node: AnyNode,
	scope: Scope,
	opts: EvalOptions = {},
): EvalResult {
	const cached = cache.get(node);
	if (cached !== undefined) {
		return cached;
	}

	let res: EvalResult = BAILED;

	switch (node.type) {
		case "JSUnaryExpression": {
			res = evalUnaryExpression(node, scope, opts);
			break;
		}

		case "JSBinaryExpression": {
			res = evalBinaryExpression(node, scope, opts);
			break;
		}

		case "JSNullLiteral": {
			res = createResult(null);
			break;
		}

		case "JSStringLiteral":
		case "JSBooleanLiteral":
		case "JSNumericLiteral": {
			res = createResult(node.value);
			break;
		}

		case "JSReferenceIdentifier": {
			const binding = scope.getBinding(node.name);
			if (binding === undefined && node.name === "undefined") {
				res = createResult(undefined);
			} else {
				if (binding instanceof ConstBinding && binding.value !== undefined) {
					res = tryStaticEvaluation(binding.value, binding.scope, opts);
				}
			}
			break;
		}

		case "JSTemplateLiteral": {
			res = evalTemplateLiteral(node, scope, opts);
			break;
		}
	}

	cache.set(node, res);
	return res;
}
