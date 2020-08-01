/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyNode,
	JSArrowFunctionExpression,
	JSAssignmentExpression,
	JSBinaryExpression,
	JSClassExpression,
	JSConditionalExpression,
	JSDoExpression,
	JSLogicalExpression,
	JSMemberExpression,
	JSObjectExpression,
	JSOptionalCallExpression,
	JSSequenceExpression,
	JSSpreadElement,
	JSSpreadProperty,
	JSUnaryExpression,
	JSUpdateExpression,
	JSYieldExpression,
	TSInferType,
	TSUnionTypeAnnotation,
} from "@internal/ast";
import {
	getPrecedence,
	isBinary,
	isConditional,
	isFor,
	isUnaryLike,
} from "@internal/js-ast-utils";

function isClassExtendsClause(node: AnyNode, parent: AnyNode): boolean {
	return (
		(parent.type === "JSClassDeclaration" || parent.type === "JSClassExpression") &&
		parent.meta.superClass === node
	);
}

const parens: Map<
	AnyNode["type"],
	(
		// rome-ignore lint/ts/noExplicitAny
		node: any,
		parent: AnyNode,
		printStack: Array<AnyNode>,
	) => boolean
> = new Map();
export default parens;

parens.set("TSAsExpression", () => true);

parens.set("TSAssignmentAsExpression", () => true);

parens.set("TSTypeAssertion", () => true);

parens.set(
	"JSMemberExpression",
	(node: JSMemberExpression, parent: AnyNode): boolean => {
		if (node.property.optional) {
			return (
				(parent.type === "JSCallExpression" && parent.callee === node) ||
				(parent.type === "JSMemberExpression" && parent.object === node)
			);
		} else {
			return false;
		}
	},
);

parens.set(
	"JSUpdateExpression",
	(node: JSUpdateExpression, parent: AnyNode): boolean => {
		return (
			// (foo++).test(), (foo++)[0]
			(parent.type === "JSMemberExpression" && parent.object === node) ||
			// (foo++)()
			(parent.type === "JSCallExpression" && parent.callee === node) ||
			// new (foo++)()
			(parent.type === "JSNewExpression" && parent.callee === node) ||
			isClassExtendsClause(node, parent)
		);
	},
);

parens.set(
	"JSObjectExpression",
	(
		node: JSObjectExpression,
		parent: AnyNode,
		printStack: Array<AnyNode>,
	): boolean => {
		return isFirstInStatement(printStack, {considerArrow: true});
	},
);

parens.set(
	"JSDoExpression",
	(node: JSDoExpression, parent: AnyNode, printStack: Array<AnyNode>): boolean => {
		return isFirstInStatement(printStack);
	},
);

function needsParenLogicalExpression(
	node: JSBinaryExpression | JSLogicalExpression,
	parent: AnyNode,
): boolean {
	if (
		node.operator === "**" &&
		parent.type === "JSBinaryExpression" &&
		parent.operator === "**"
	) {
		return parent.left === node;
	}

	// class A extends (B ?? C) {
	if (isClassExtendsClause(node, parent)) {
		return true;
	}

	// (f ?? g)()
	// (f ?? g)?.()
	// new (A ?? B)()
	if (
		parent.type === "JSCallExpression" ||
		parent.type === "JSOptionalCallExpression" ||
		parent.type === "JSNewExpression"
	) {
		return parent.callee === node;
	}

	// ...(a ?? b)
	// await (a ?? b)
	if (isUnaryLike(parent) || parent.type === "JSAwaitExpression") {
		return true;
	}

	// (a ?? b).x
	// (a ?? b)?.x
	if (parent.type === "JSMemberExpression" && parent.object === node) {
		return true;
	}

	// (a ?? b) ?? c
	// a ?? (b ?? c)
	if (parent.type === "JSLogicalExpression") {
		if (node.type === "JSLogicalExpression") {
			return node.operator !== parent.operator;
		}
	}

	if (isBinary(parent)) {
		const parentOp = parent.operator;
		const parentPos = getPrecedence(parentOp);

		const nodeOp = node.operator;
		const nodePos = getPrecedence(nodeOp);

		if (
			// Logical expressions with the same precedence don't need parens.
			(parentPos === nodePos &&
			parent.right === node &&
			parent.type !== "JSLogicalExpression") ||
			parentPos > nodePos
		) {
			return true;
		}
	}

	return false;
}

parens.set("JSLogicalExpression", needsParenLogicalExpression);

parens.set(
	"JSBinaryExpression",
	(node: JSBinaryExpression, parent: AnyNode): boolean => {
		// let i = (1 in []);
		// for ((1 in []);;);
		if (
			node.operator === "in" &&
			(parent.type === "JSVariableDeclarator" || isFor(parent))
		) {
			return true;
		}

		return needsParenLogicalExpression(node, parent);
	},
);

parens.set(
	"JSSequenceExpression",
	(node: JSSequenceExpression, parent: AnyNode): boolean => {
		if (
			// Although parentheses wouldn't hurt around sequence
			// expressions in the head of for loops, traditional style
			// dictates that e.g. i++, j++ should not be wrapped with
			// parentheses.
			parent.type === "JSForStatement" ||
			parent.type === "JSThrowStatement" ||
			parent.type === "JSReturnStatement" ||
			(parent.type === "JSIfStatement" && parent.test === node) ||
			(parent.type === "JSWhileStatement" && parent.test === node) ||
			(parent.type === "JSForInStatement" && parent.right === node) ||
			(parent.type === "JSSwitchStatement" && parent.discriminant === node) ||
			(parent.type === "JSExpressionStatement" && parent.expression === node)
		) {
			return false;
		}

		// Arrow function builder handles the parens printing.
		if (parent.type === "JSArrowFunctionExpression") {
			return false;
		}

		// Otherwise err on the side of overparenthesization, adding
		// explicit exceptions above if this proves overzealous.
		return true;
	},
);

function needsParenYieldExpression(
	node: JSYieldExpression,
	parent: AnyNode,
): boolean {
	return (
		isBinary(parent) ||
		isUnaryLike(parent) ||
		parent.type === "JSMemberExpression" ||
		(parent.type === "JSCallExpression" && parent.callee === node) ||
		(parent.type === "JSNewExpression" && parent.callee === node) ||
		(parent.type === "JSAwaitExpression" && node.type === "JSYieldExpression") ||
		(parent.type === "JSConditionalExpression" && node === parent.test) ||
		isClassExtendsClause(node, parent)
	);
}

parens.set("JSYieldExpression", needsParenYieldExpression);
parens.set("JSAwaitExpression", needsParenYieldExpression);

parens.set(
	"JSOptionalCallExpression",
	(node: JSOptionalCallExpression, parent: AnyNode): boolean => {
		return (
			(parent.type === "JSCallExpression" && parent.callee === node) ||
			(parent.type === "JSMemberExpression" && parent.object === node)
		);
	},
);

parens.set(
	"JSClassExpression",
	(
		node: JSClassExpression,
		parent: AnyNode,
		printStack: Array<AnyNode>,
	): boolean => {
		return isFirstInStatement(printStack, {considerDefaultExports: true});
	},
);

function needsParenUnaryExpression(
	node:
		| JSUnaryExpression
		| JSArrowFunctionExpression
		| JSAssignmentExpression
		| JSConditionalExpression
		| JSSpreadElement
		| JSSpreadProperty,
	parent: AnyNode,
): boolean {
	return (
		(parent.type === "JSMemberExpression" && parent.object === node) ||
		(parent.type === "JSCallExpression" && parent.callee === node) ||
		(parent.type === "JSNewExpression" && parent.callee === node) ||
		(parent.type === "JSBinaryExpression" &&
		parent.operator === "**" &&
		parent.left === node) ||
		isClassExtendsClause(node, parent)
	);
}

parens.set("JSUnaryExpression", needsParenUnaryExpression);
parens.set("JSSpreadElement", needsParenUnaryExpression);
parens.set("JSSpreadProperty", needsParenUnaryExpression);

parens.set(
	"JSFunctionExpression",
	(node: AnyNode, parent: AnyNode, printStack: Array<AnyNode>): boolean => {
		return isFirstInStatement(printStack, {considerDefaultExports: true});
	},
);

parens.set(
	"JSArrowFunctionExpression",
	(node: JSArrowFunctionExpression, parent: AnyNode): boolean => {
		return (
			parent.type === "JSExportLocalDeclaration" ||
			needsParenConditionalExpression(node, parent)
		);
	},
);

function needsParenConditionalExpression(
	node:
		| JSArrowFunctionExpression
		| JSAssignmentExpression
		| JSConditionalExpression,
	parent: AnyNode,
): boolean {
	if (
		isUnaryLike(parent) ||
		isBinary(parent) ||
		(parent.type === "JSConditionalExpression" && parent.test === node) ||
		parent.type === "JSAwaitExpression" ||
		(parent.type === "JSMemberExpression" &&
		parent.object === node &&
		parent.property.optional) ||
		(parent.type === "JSOptionalCallExpression" && parent.callee === node) ||
		parent.type === "JSTaggedTemplateExpression" ||
		parent.type === "TSTypeAssertion" ||
		parent.type === "TSAsExpression"
	) {
		return true;
	}

	return needsParenUnaryExpression(node, parent);
}

parens.set("JSConditionalExpression", needsParenConditionalExpression);

parens.set(
	"JSAssignmentExpression",
	(node: JSAssignmentExpression, parent: AnyNode): boolean => {
		if (node.left.type === "JSAssignmentObjectPattern") {
			return true;
		} else {
			return needsParenConditionalExpression(node, parent);
		}
	},
);

function needsParenUnionTypeAnnotation(
	node: TSUnionTypeAnnotation,
	parent: AnyNode,
) {
	return (
		parent.type === "TSIntersectionTypeAnnotation" ||
		parent.type === "TSUnionTypeAnnotation" ||
		parent.type === "TSArrayType" ||
		(parent.type === "TSTupleElement" && parent.optional === true)
	);
}

parens.set("TSUnionTypeAnnotation", needsParenUnionTypeAnnotation);
parens.set("TSIntersectionTypeAnnotation", needsParenUnionTypeAnnotation);

parens.set(
	"TSInferType",
	(node: TSInferType, parent: AnyNode): boolean => {
		return (
			parent.type === "TSArrayType" ||
			(parent.type === "TSTupleElement" && parent.optional === true)
		);
	},
);

// Walk up the print stack to deterimine if our node can come first
// in statement.
function isFirstInStatement(
	printStack: Array<AnyNode>,
	{considerArrow = false, considerDefaultExports = false} = {},
): boolean {
	let i = printStack.length - 1;
	let node = printStack[i];
	i--;
	let parent = printStack[i];
	while (i > 0) {
		if (
			(parent.type === "JSExpressionStatement" && parent.expression === node) ||
			parent.type === "JSTaggedTemplateExpression" ||
			(considerDefaultExports &&
			parent.type === "JSExportDefaultDeclaration" &&
			parent.declaration === node) ||
			(considerArrow &&
			parent.type === "JSArrowFunctionExpression" &&
			parent.body === node)
		) {
			return true;
		}

		if (
			(parent.type === "JSCallExpression" && parent.callee === node) ||
			(parent.type === "JSSequenceExpression" && parent.expressions[0] === node) ||
			(parent.type === "JSMemberExpression" && parent.object === node) ||
			(isConditional(parent) && parent.test === node) ||
			(isBinary(parent) && parent.left === node) ||
			(parent.type === "JSAssignmentExpression" && parent.left === node)
		) {
			node = parent;
			i--;
			parent = printStack[i];
		} else {
			return false;
		}
	}

	return false;
}
