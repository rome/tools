import {
	AnyJSExpression,
	AnyNode,
	JSBinaryExpression,
	JSCallExpression,
	JSConditionalExpression,
	JSIfStatement,
	JSMemberExpression,
	JSNullLiteral,
	JSReferenceIdentifier,
	jsIdentifier,
	jsMemberExpression,
	jsOptionalCallExpression,
	jsStaticMemberProperty,
} from "@internal/ast";
import {createLintVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";

function isUndefined(node: AnyJSExpression): node is JSReferenceIdentifier {
	return node.type === "JSReferenceIdentifier" && node.name === "undefined";
}

function isNullOrUndefined(
	node: AnyJSExpression,
): node is JSNullLiteral | JSReferenceIdentifier {
	return node.type === "JSNullLiteral" || isUndefined(node);
}

function isReferenceOrMemberExpression(
	node: AnyJSExpression,
): node is JSReferenceIdentifier | JSMemberExpression {
	return (
		node.type === "JSReferenceIdentifier" || node.type === "JSMemberExpression"
	);
}

type Output = {
	name: string;
	optional?: boolean;
};

/**
 * Arrayify object to by identifiers
 *
 * input
 * `foo.bar?.baz`
 *
 * output
 * `[{name: "foo"}, {name: "bar"}, {name: "baz", optional: true}]`
 *
 * input
 * `foo[bar ? 'baz' : 'zoo']`
 *
 * output
 * `null`
 */
function memberExpressionToArray(
	arg: JSMemberExpression | JSReferenceIdentifier,
): Output[] | null {
	let node: AnyJSExpression = arg;
	const result = [];
	while (true) {
		if (node.type === "JSReferenceIdentifier") {
			result.unshift({name: node.name});
			return result;
		}
		if (node.type !== "JSMemberExpression") {
			return null;
		}
		const {property} = node;

		if (property.type === "JSComputedMemberProperty") {
			return null;
		}
		if (property.value.type === "JSPrivateName") {
			return null;
		}

		result.unshift({
			name: property.value.name,
			optional: property.optional,
		});
		node = node.object;
	}
}

function mergeMemberExpressions(
	left: JSMemberExpression | JSReferenceIdentifier,
	right: JSMemberExpression | JSReferenceIdentifier,
	options: {
		inclusive: boolean;
	},
):
	| {
			node: JSMemberExpression | JSReferenceIdentifier;
			sameLength: boolean;
		}
	| null {
	const leftArr = memberExpressionToArray(left);
	const rightArr = memberExpressionToArray(right);

	if (leftArr === null || rightArr === null) {
		return null;
	}

	/**
	 * it is okay for left and right to be the same for optional call expr
	 * it is not okay for left and right to be the same for optional member expr
	 */
	if (
		options.inclusive
			? leftArr.length > rightArr.length
			: leftArr.length >= rightArr.length
	) {
		return null;
	}

	if (!leftArr.every((el, i) => el.name === rightArr[i].name)) {
		return null;
	}

	const diff = rightArr.slice(leftArr.length);

	let result = left;

	for (let i = 0; i < diff.length; i++) {
		result = jsMemberExpression.create({
			object: result,
			property: jsStaticMemberProperty.create({
				value: jsIdentifier.quick(diff[i].name),
				optional: i === 0 || diff[i].optional,
			}),
		});
	}

	return {
		node: result,
		sameLength: leftArr.length === rightArr.length,
	};
}

function getVerifiedLeft(
	node: AnyJSExpression,
): JSMemberExpression | JSReferenceIdentifier | null {
	if (isReferenceOrMemberExpression(node)) {
		return node;
	}

	if (
		node.type === "JSBinaryExpression" &&
		(node.operator === "!==" || node.operator === "!=")
	) {
		if (
			isNullOrUndefined(node.left) &&
			isReferenceOrMemberExpression(node.right)
		) {
			return node.right;
		}

		if (
			isNullOrUndefined(node.right) &&
			isReferenceOrMemberExpression(node.left)
		) {
			return node.left;
		}
	}

	return null;
}

type VerifiedRight = {
	node: JSMemberExpression;
	build: (expr: JSMemberExpression) => JSBinaryExpression | JSMemberExpression;
};
function getVerifiedRight(node: AnyJSExpression): null | VerifiedRight {
	if (node.type === "JSMemberExpression") {
		return {
			node,
			build: (node) => node,
		};
	}

	if (
		node.type === "JSBinaryExpression" &&
		// we cannot automatically replace
		// `foo !== undefined && foo.bar !== 'some_var'`
		// for false positives reason
		(((node.operator === "!==" || node.operator === "!=") &&
		(isNullOrUndefined(node.left) || isNullOrUndefined(node.right))) ||
		node.operator === "===" ||
		node.operator === "==")
	) {
		if (node.left.type === "JSMemberExpression") {
			return {
				node: node.left,
				build: (arg) => ({
					...node,
					left: arg,
				}),
			};
		}

		if (node.right.type === "JSMemberExpression") {
			return {
				node: node.right,
				build: (arg) => ({
					...node,
					right: arg,
				}),
			};
		}
	}

	return null;
}

function getVerifiedConsequent(node: AnyNode): null | JSCallExpression {
	if (
		node.type === "JSExpressionStatement" &&
		node.expression.type === "JSCallExpression"
	) {
		return node.expression;
	}

	if (
		node.type === "JSBlockStatement" &&
		node.body.length === 1 &&
		node.body[0].type === "JSExpressionStatement" &&
		node.body[0].expression.type === "JSCallExpression"
	) {
		return node.body[0].expression;
	}

	return null;
}

function extractIfStatementComments(node: JSIfStatement): string[] {
	const result: string[] = [];

	if (node.leadingComments) {
		result.push(...node.leadingComments);
	}

	if (node.test.leadingComments) {
		result.push(...node.test.leadingComments);
	}

	if (node.consequent.type === "JSBlockStatement") {
		if (node.consequent.leadingComments) {
			result.push(...node.consequent.leadingComments);
		}

		const head = node.consequent.body[0];

		if (head?.leadingComments) {
			result.push(...head.leadingComments);
		}
	}

	return result;
}

type NodeComments = {
	leadingComments: string[];
	trailingComments: string[];
};
function extractTernaryComments(node: JSConditionalExpression): NodeComments {
	const result: NodeComments = {
		leadingComments: [],
		trailingComments: [],
	};

	// ternary leading comments should remain leading
	if (node.leadingComments) {
		result.leadingComments.push(...node.leadingComments);
	}

	if (node.test.leadingComments) {
		result.leadingComments.push(...node.test.leadingComments);
	}
	if (node.test.trailingComments) {
		result.leadingComments.push(...node.test.trailingComments);
	}

	if (node.consequent.leadingComments) {
		result.leadingComments.push(...node.consequent.leadingComments);
	}
	if (node.consequent.trailingComments) {
		result.leadingComments.push(...node.consequent.trailingComments);
	}

	if (node.alternate.leadingComments) {
		result.trailingComments.push(...node.alternate.leadingComments);
	}
	if (node.alternate.trailingComments) {
		result.trailingComments.push(...node.alternate.trailingComments);
	}

	// ternary trailing comments should remain trailing
	if (node.trailingComments) {
		result.trailingComments.push(...node.trailingComments);
	}

	return result;
}

export default createLintVisitor({
	name: "js/preferOptionalChaining",
	enter(path) {
		const {node} = path;

		/**
		 * Optional call expression
		 * `if (foo) foo()` --> `foo?.()`
		 */
		if (node.type === "JSIfStatement" && node.alternate === undefined) {
			const consequent = getVerifiedConsequent(node.consequent);
			const left = getVerifiedLeft(node.test);
			if (
				left !== null &&
				consequent !== null &&
				isReferenceOrMemberExpression(consequent.callee)
			) {
				const newCallee = mergeMemberExpressions(
					left,
					consequent.callee,
					{inclusive: true},
				);
				if (newCallee) {
					return path.addFixableDiagnostic(
						{
							fixed: signals.replace(
								jsOptionalCallExpression.create({
									...consequent,
									callee: newCallee.node,
									leadingComments: extractIfStatementComments(node),
									trailingComments: node.trailingComments || [],
									optional: newCallee.sameLength,
								}),
							),
						},
						descriptions.LINT.JS_PREFER_OPTIONAL_CHAINING,
					);
				}
			}

			return signals.retain;
		}

		/**
		 * `foo ? foo() : undefined` --> `foo?.()`
		 * `foo ? foo.bar : undefined` --> `foo?.bar`
		 */
		if (node.type === "JSConditionalExpression" && isUndefined(node.alternate)) {
			const left = getVerifiedLeft(node.test);

			/**
			 * optional call expr
			 * `foo ? foo() : undefined` --> `foo?.()`
			 */
			if (
				node.consequent.type === "JSCallExpression" &&
				isReferenceOrMemberExpression(node.consequent.callee) &&
				left !== null
			) {
				const newCallee = mergeMemberExpressions(
					left,
					node.consequent.callee,
					{inclusive: true},
				);
				if (newCallee) {
					return path.addFixableDiagnostic(
						{
							fixed: signals.replace(
								jsOptionalCallExpression.create({
									...node.consequent,
									...extractTernaryComments(node),
									callee: newCallee.node,
									optional: newCallee.sameLength,
								}),
							),
						},
						descriptions.LINT.JS_PREFER_OPTIONAL_CHAINING,
					);
				}
			}

			/**
			 * optional member expr
			 * `foo ? foo.bar : undefined` --> `foo?.bar`
			 */
			if (node.consequent.type === "JSMemberExpression" && left !== null) {
				const right = getVerifiedRight(node.consequent);
				if (right) {
					const newMemberExpression = mergeMemberExpressions(
						left,
						right.node,
						{inclusive: false},
					);

					if (
						typeof newMemberExpression?.node.type === "string" &&
						newMemberExpression?.node.type !== "JSReferenceIdentifier"
					) {
						return path.addFixableDiagnostic(
							{
								fixed: signals.replace({
									...right.build(newMemberExpression.node),
									...extractTernaryComments(node),
								}),
							},
							descriptions.LINT.JS_PREFER_OPTIONAL_CHAINING,
						);
					}
				}
			}
		}

		/**
		 * Optional member expression and call expression
		 * `foo && foo.bar` --> `foo?.bar`
		 * `foo.bar && foo.bar()` --> `foo.bar?.()
		 */
		if (node.type === "JSLogicalExpression" && node.operator === "&&") {
			const left = getVerifiedLeft(node.left);
			if (!left) {
				return signals.retain;
			}

			if (
				node.right.type === "JSCallExpression" &&
				isReferenceOrMemberExpression(node.right.callee)
			) {
				const newCallee = mergeMemberExpressions(
					left,
					node.right.callee,
					{inclusive: true},
				);
				if (newCallee) {
					return path.addFixableDiagnostic(
						{
							fixed: signals.replace(
								jsOptionalCallExpression.create({
									...node.right,
									leadingComments: node.leadingComments || [],
									trailingComments: node.trailingComments || [],
									callee: newCallee.node,
									optional: newCallee.sameLength,
								}),
							),
						},
						descriptions.LINT.JS_PREFER_OPTIONAL_CHAINING,
					);
				}
			}

			const right = getVerifiedRight(node.right);
			if (!right) {
				return signals.retain;
			}
			const newMemberExpression = mergeMemberExpressions(
				left,
				right.node,
				{inclusive: false},
			);

			if (
				!newMemberExpression ||
				newMemberExpression.node.type === "JSReferenceIdentifier"
			) {
				return signals.retain;
			}

			return path.addFixableDiagnostic(
				{
					fixed: signals.replace({
						...right.build(newMemberExpression.node),
						leadingComments: node.leadingComments || [],
						trailingComments: node.trailingComments || [],
					}),
				},
				descriptions.LINT.JS_PREFER_OPTIONAL_CHAINING,
			);
		}

		return signals.retain;
	},
});
