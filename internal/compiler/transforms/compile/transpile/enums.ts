import {
	AnyJSExpression,
	JSBindingIdentifier,
	JSCallExpression,
	JSExpressionStatement,
	JSVariableDeclaration,
	TSEnumDeclaration,
	jsCallExpression,
	jsFunctionExpression,
	jsNumericLiteral,
	jsStringLiteral,
	jsVariableDeclaration,
	jsVariableDeclarationStatement,
	jsVariableDeclarator,
} from "@internal/ast";
import {
	CompilerContext,
	Path,
	Scope,
	createVisitor,
	signals,
} from "@internal/compiler";

import {descriptions} from "@internal/diagnostics";
import {template, tryStaticEvaluation} from "@internal/js-ast-utils";
import {EvalResult} from "@internal/js-ast-utils/tryStaticEvaluation";

type PreviousEnumMembers = Map<string, EvalResult["value"]>;

function buildEnumWrapper(
	path: Path,
	node: TSEnumDeclaration,
	id: JSBindingIdentifier,
	assignments: Array<AnyJSExpression>,
): JSVariableDeclaration {
	const call = jsCallExpression.assert(
		template.expression`(function () { const ${id} = {}; return ${id};})();`,
	);

	const statements: Array<JSExpressionStatement> = assignments.map((expression) => ({
		type: "JSExpressionStatement",
		expression,
	}));

	const func = jsFunctionExpression.assert(call.callee);
	const factoryCall: JSCallExpression = {
		...call,
		callee: {
			...func,
			body: {
				...func.body,
				body: [func.body.body[0], ...statements, func.body.body[1]],
			},
		},
	};

	return jsVariableDeclaration.create({
		kind: "const",
		declarations: [
			jsVariableDeclarator.create({
				id: node.id,
				init: factoryCall,
			}),
		],
	});
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
	path: Path,
	node: TSEnumDeclaration,
	scope: Scope,
	context: CompilerContext,
): JSVariableDeclaration {
	const x = translateEnumValues(node, scope, context);
	const assignments = x.map(([memberName, memberValue]) =>
		buildEnumMember(
			typeof memberValue !== "string" && memberValue.type === "JSStringLiteral",
			{...node.id},
			memberName,
			memberValue,
		)
	);
	return buildEnumWrapper(path, node, {...node.id}, assignments);
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

export default createVisitor({
	name: "enums",
	enter(path) {
		const {context, node, scope} = path;

		if (node.type !== "TSEnumDeclaration") {
			return signals.retain;
		}

		if (node.const) {
			context.addNodeDiagnostic(
				node,
				descriptions.COMPILER.CONST_ENUMS_UNSUPPORTED,
			);
			return signals.remove;
		}

		if (node.declare) {
			return signals.remove;
		}

		const fill = enumFill(path, node, scope, context);

		switch (path.parent.type) {
			case "JSExportLocalDeclaration":
				return signals.replace(fill);

			case "JSBlockStatement":
			case "JSRoot":
				return signals.replace(jsVariableDeclarationStatement.quick(fill));

			default:
				throw new Error(`Unexpected enum parent '${path.parent.type}`);
		}
	},
});
