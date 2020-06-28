import {
	AnyJSExpression,
	JSBindingIdentifier,
	JSCallExpression,
	TSEnumDeclaration,
	jsCallExpression,
	jsFunctionExpression,
	jsNumericLiteral,
	jsStringLiteral,
} from "@romejs/ast";
import {
	CompilerContext,
	LetBinding,
	Path,
	Scope,
	VarBinding,
} from "@romejs/compiler";
import {REDUCE_REMOVE} from "@romejs/compiler/constants";
import {descriptions} from "@romejs/diagnostics";
import {template, tryStaticEvaluation} from "@romejs/js-ast-utils";
import {EvalResult} from "@romejs/js-ast-utils/tryStaticEvaluation";

type PreviousEnumMembers = Map<string, EvalResult["value"]>;

function buildEnumWrapper(
	id: JSBindingIdentifier,
	assignments: Array<AnyJSExpression>,
): JSCallExpression {
	const call = jsCallExpression.assert(
		template.expression`(function (${id}) {})(${id} || (${id} = {}));`,
	);

	const func = jsFunctionExpression.assert(call.callee);

	return {
		...call,
		callee: {
			...func,
			body: {
				...func.body,
				body: assignments.map((expression) => ({
					type: "JSExpressionStatement",
					expression,
				})),
			},
		},
	};
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

function enumFill(
	node: TSEnumDeclaration,
	scope: Scope,
	context: CompilerContext,
): AnyJSExpression {
	const x = translateEnumValues(node, scope, context);
	const assignments = x.map(([memberName, memberValue]) =>
		buildEnumMember(
			typeof memberValue !== "string" && memberValue.type === "JSStringLiteral",
			{...node.id},
			memberName,
			memberValue,
		)
	);
	return buildEnumWrapper({...node.id}, assignments);
}

function translateEnumValues(
	node: TSEnumDeclaration,
	scope: Scope,
	context: CompilerContext,
): Array<[string, AnyJSExpression]> {
	const seen: PreviousEnumMembers = new Map();
	let prev: number | undefined = -1;

	return node.members.map((member) => {
		let value: AnyJSExpression;
		const initializer = member.initializer;
		const name =
			member.id.type === "JSIdentifier" ? member.id.name : member.id.value;

		if (initializer) {
			let {value: constValue, bailed} = tryStaticEvaluation(
				initializer,
				scope,
				{
					isNodeValid: (node, resolvedNode) => {
						if (
							node.type === "JSReferenceIdentifier" &&
							resolvedNode.type !== "JSNumericLiteral"
						) {
							context.addNodeDiagnostic(
								member,
								descriptions.COMPILER.ENUM_COMPUTED_VALUES_UNSUPPORTED,
							);
							return false;
						}
						return true;
					},
				},
			);

			if (bailed && initializer.type === "JSReferenceIdentifier") {
				constValue = seen.get(initializer.name);
			}
			if (constValue !== undefined) {
				seen.set(name, constValue);

				if (typeof constValue === "number") {
					value = jsNumericLiteral.create({value: constValue});
					prev = constValue;
				} else {
					value = jsStringLiteral.create({value: String(constValue)});
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
				seen.set(name, prev);
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
		const {context, node, scope} = path;

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

		const fill = enumFill(node, scope, context);

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
