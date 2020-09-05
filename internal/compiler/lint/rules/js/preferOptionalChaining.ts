import {
	AnyJSExpression,
	JSBinaryExpression,
	JSMemberExpression,
	JSNullLiteral,
	JSReferenceIdentifier,
	jsIdentifier,
	jsMemberExpression,
	jsStaticMemberProperty,
} from "@internal/ast";
import {createVisitor, signals} from "@internal/compiler";
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
):
	| Array<{
			name: string;
			optional?: boolean;
		}>
	| null {
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

export default createVisitor({
	name: "js/preferOptionalChaining",
	enter(path) {
		const {node} = path;

		/**
		 * `foo ? foo.bar : undefined` --> `foo?.bar`
		 */
		if (node.type === "JSConditionalExpression" && isUndefined(node.alternate)) {
			const left = getVerifiedLeft(node.test);
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
								fixed: signals.replace(right.build(newMemberExpression.node)),
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
		 */
		if (node.type === "JSLogicalExpression" && node.operator === "&&") {
			const left = getVerifiedLeft(node.left);
			if (!left) {
				return signals.retain;
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
					fixed: signals.replace(right.build(newMemberExpression.node)),
				},
				descriptions.LINT.JS_PREFER_OPTIONAL_CHAINING,
			);
		}

		return signals.retain;
	},
});
