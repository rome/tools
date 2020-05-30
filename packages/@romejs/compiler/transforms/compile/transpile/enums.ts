import {
	AnyJSExpression,
	AnyJSStatement,
	JSBindingIdentifier,
	JSCallExpression,
	JSFunctionExpression,
	TSEnumDeclaration,
	jsNumericLiteral,
	jsStringLiteral,
} from "@romejs/ast";
import {LetBinding, Path, VarBinding} from "@romejs/compiler";
import {REDUCE_REMOVE} from "@romejs/compiler/constants";
import {descriptions} from "@romejs/diagnostics";
import {template, tryStaticEvaluation} from "@romejs/js-ast-utils";

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
	return isString
		? buildStringAssignment(id, name, value)
		: buildNumericAssignment(id, name, value);
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
			let {value: constValue, bailed} = tryStaticEvaluation(initializer);
			if (bailed && initializer.type === "JSReferenceIdentifier") {
				constValue = seen[initializer.name];
			}
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

export default {
	name: "enums",
	enter(path: Path) {
		const {context, node} = path;

		if (node.type !== "TSEnumDeclaration") {
			return node;
		}

		if (node.const) {
			context.addNodeDiagnostic(
				node,
				descriptions.COMPILER.CONST_ENUMS_UNSUPPORTED,
			);
			return REDUCE_REMOVE;
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
