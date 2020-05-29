import {
	AnyJSExpression,
	AnyJSStatement,
	JSBinaryExpression,
	JSBindingIdentifier,
	JSCallExpression,
	JSFunctionExpression,
	JSUnaryExpression,
	TSEnumDeclaration,
	jsNumericLiteral,
	jsStringLiteral,
} from "@romejs/ast";
import {LetBinding, Path, VarBinding} from "@romejs/compiler";
import {REDUCE_REMOVE} from "@romejs/compiler/constants";
import {template} from "@romejs/js-ast-utils";

interface PreviousEnumMembers {
	[name: string]: number | string;
}

function buildEnumWrapper(
	id: JSBindingIdentifier,
	assignments: Array<AnyJSExpression>,
): AnyJSExpression {
	const expressionStatement = (template.expression`
		(function (${id}) {})(${id} || (${id} = {}));
	` as JSCallExpression);
	const functionExpression = (expressionStatement.callee as JSFunctionExpression);
	functionExpression.body.body = (assignments as Array<AnyJSStatement>);
	return (expressionStatement as AnyJSExpression);
}

function buildEnumMember(
	isString: boolean,
	id: JSBindingIdentifier,
	name: string,
	value: AnyJSExpression,
): AnyJSExpression {
	return (isString ? buildStringAssignment : buildNumericAssignment)(
		id,
		name,
		value,
	);
}

function buildNumericAssignment(
	id: JSBindingIdentifier,
	name: string,
	value: AnyJSExpression,
): AnyJSExpression {
	const nameNode = jsStringLiteral.create({value: name});
	return template.expression`
		${id}[${id}[${nameNode}] = ${value}] = ${nameNode};
	`;
}

function buildStringAssignment(
	id: JSBindingIdentifier,
	name: string,
	value: AnyJSExpression,
): AnyJSExpression {
	const nameNode = jsStringLiteral.create({value: name});
	return template.expression`
		${id}[${nameNode}] = ${value};
	`;
}

function enumFill(node: TSEnumDeclaration): AnyJSExpression {
	const x = translateEnumValues(node);
	const assignments = x.map(([memberName, memberValue]) =>
		buildEnumMember(
			typeof memberValue !== "string" && memberValue.type === "JSStringLiteral",
			({...node.id} as JSBindingIdentifier),
			memberName,
			memberValue,
		)
	);
	return buildEnumWrapper(({...node.id} as JSBindingIdentifier), assignments);
}

function translateEnumValues(
	node: TSEnumDeclaration,
): Array<[string, AnyJSExpression]> {
	const seen: PreviousEnumMembers = Object.create(null);
	let prev: number | undefined = -1;
	return node.members.map((member) => {
		let value: AnyJSExpression;
		const initializer = member.initializer;
		const name =
			member.id.type === "JSIdentifier" ? member.id.name : member.id.value;

		if (initializer) {
			const constValue = evaluate(initializer, seen);
			if (constValue !== undefined) {
				seen[name] = constValue;
				if (typeof constValue === "number") {
					value = jsNumericLiteral.create({value: constValue});
					prev = constValue;
				} else {
					value = jsStringLiteral.create({value: constValue});
					prev = undefined;
				}
			} else {
				value = initializer;
				prev = undefined;
			}
		} else {
			if (prev !== undefined) {
				prev++;
				value = jsNumericLiteral.create({value: prev});
				seen[name] = prev;
			} else {
				throw new Error("Enum member must have initializer");
			}
		}

		return [name, value];
	});
}

function evaluate(
	expr: AnyJSExpression,
	seen: PreviousEnumMembers,
): number | string | undefined {
	return evalConstant(expr);

	function evalConstant(expr: AnyJSExpression): number | string | undefined {
		switch (expr.type) {
			case "JSStringLiteral":
				return expr.value;
			case "JSUnaryExpression":
				return evalUnaryExpression(expr);
			case "JSBinaryExpression":
				return evalBinaryExpression(expr);
			case "JSNumericLiteral":
				return expr.value;
			case "JSReferenceIdentifier":
				return seen[expr.name];
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
}

export default {
	name: "enums",
	enter(path: Path) {
		const {node} = path;

		if (node.type !== "TSEnumDeclaration") {
			return node;
		}

		if (node.const) {
			throw new Error('"const" enums are not supported');
		}

		if (node.declare) {
			return REDUCE_REMOVE;
		}

		const fill = enumFill(node);

		switch (path.parent.type) {
			case "JSBlockStatement":
			case "JSExportLocalDeclaration":
			case "JSRoot": {
				const BindingCtor =
					path.parent.type === "JSRoot" ? VarBinding : LetBinding;
				path.scope.addBinding(
					new BindingCtor({
						node: node.id,
						name: node.id.name,
						scope: path.scope,
					}),
				);
				return fill;
			}

			default:
				throw new Error(`Unexpected enum parent '${path.parent.type}`);
		}
	},
};
