import {
	AnyNode,
	JSBinaryExpression,
	JSTemplateLiteral,
	JSUnaryExpression,
} from "@internal/ast";
import {Scope} from "@internal/compiler";
import {resolveIndirection} from "./resolveIndirection";

export type EvalResult = {
	value: undefined | null | bigint | string | number | boolean;
	bailed: boolean;
};

export type EvalOptions = {
	isNodeValid?: (node: AnyNode, resolvedNode: AnyNode) => boolean;
};

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
		// rome-ignore lint/ts/noExplicitAny
		const value = (res.value as any);

		switch (expr.operator) {
			case "+":
				return createResult(+value);

			case "-":
				return createResult(-value);

			case "~":
				return createResult(~value);

			case "!":
				return createResult(!value);
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
	// rome-ignore lint/ts/noExplicitAny
	const l = (left.value as any);
	// rome-ignore lint/ts/noExplicitAny
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

export function tryStaticEvaluation(
	node: AnyNode,
	scope: Scope,
	opts: EvalOptions = {},
): EvalResult {
	const cached = cache.get(node);
	if (cached !== undefined) {
		return cached;
	}

	let res: EvalResult = BAILED;

	const {node: resolvedNode, scope: resolvedScope} = resolveIndirection(
		node,
		scope,
	);

	if (opts.isNodeValid && !opts.isNodeValid(node, resolvedNode)) {
		return res;
	}

	switch (resolvedNode.type) {
		case "JSUnaryExpression": {
			res = evalUnaryExpression(resolvedNode, resolvedScope, opts);
			break;
		}

		case "JSBinaryExpression": {
			res = evalBinaryExpression(resolvedNode, resolvedScope, opts);
			break;
		}

		case "JSNullLiteral": {
			res = createResult(null);
			break;
		}

		case "JSBigIntLiteral": {
			res = createResult(BigInt(resolvedNode.value));
			break;
		}

		case "JSStringLiteral":
		case "JSBooleanLiteral":
		case "JSNumericLiteral": {
			res = createResult(resolvedNode.value);
			break;
		}

		case "JSReferenceIdentifier": {
			const binding = resolvedScope.getBinding(resolvedNode.name);
			if (binding === undefined && resolvedNode.name === "undefined") {
				res = createResult(undefined);
			}
			break;
		}

		case "JSTemplateLiteral": {
			res = evalTemplateLiteral(resolvedNode, resolvedScope, opts);
			break;
		}
	}

	cache.set(node, res);
	return res;
}
