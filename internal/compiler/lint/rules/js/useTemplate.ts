/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {createVisitor, signals} from "@internal/compiler";
import {
	AnyJSExpression,
	AnyNode,
	JSBinaryExpression,
	JSStringLiteral,
	JSTemplateElement,
	JSTemplateLiteral,
	jsStringLiteral,
	jsTemplateElement,
	jsTemplateLiteral,
} from "@internal/ast";
import {descriptions} from "@internal/diagnostics";

type TemplatePart = AnyJSExpression | JSTemplateElement;
type StaticString = JSStringLiteral | JSTemplateElement;

// expr + expr
function isBinaryAddExpression(node: AnyNode): node is JSBinaryExpression {
	return node.type === "JSBinaryExpression" && node.operator === "+";
}

// 'str' + 'str'
// 'str' + expr
// expr + 'str'
// expr + (expr + 'str')
// (expr + 'str') + expr
function isUnnecessaryStringConcatExpression(
	node: AnyNode,
): node is JSBinaryExpression {
	if (!isBinaryAddExpression(node)) {
		return false;
	}

	if (node.left.type === "JSBinaryExpression") {
		if (isUnnecessaryStringConcatExpression(node.left)) {
			return true;
		}
	}

	if (node.right.type === "JSBinaryExpression") {
		if (isUnnecessaryStringConcatExpression(node.right)) {
			return true;
		}
	}

	if (
		node.left.type === "JSStringLiteral" &&
		!node.left.value.includes("`") &&
		!node.left.value.includes("\n")
	) {
		return true;
	}

	if (
		node.right.type === "JSStringLiteral" &&
		!node.right.value.includes("`") &&
		!node.right.value.includes("\n")
	) {
		return true;
	}

	if (node.left.type === "JSTemplateLiteral") {
		return true;
	}

	if (node.right.type === "JSTemplateLiteral") {
		return true;
	}

	return false;
}

// expr + expr + expr + ...
function collectBinaryAddExpressionExpressions(
	node: JSBinaryExpression,
): Array<AnyJSExpression> {
	let expressions: Array<AnyJSExpression> = [];

	if (isBinaryAddExpression(node.left)) {
		expressions = expressions.concat(
			collectBinaryAddExpressionExpressions(node.left),
		);
	} else {
		expressions.push(node.left);
	}

	if (isBinaryAddExpression(node.right)) {
		expressions = expressions.concat(
			collectBinaryAddExpressionExpressions(node.right),
		);
	} else {
		expressions.push(node.right);
	}

	return expressions;
}

// zips template.quasis and template.expressions into one array
function zipTemplateLiteralParts(
	template: JSTemplateLiteral,
): Array<TemplatePart> {
	let templateParts = [];

	for (let i = 0; i < template.quasis.length; i++) {
		templateParts.push(template.quasis[i]);

		if (i + 1 < template.quasis.length) {
			templateParts.push(template.expressions[i]);
		}
	}

	return templateParts;
}

// flattens an array of expressions into TemplateLiteral parts
function flattenExpressionsToTemplateParts(
	expressions: Array<AnyJSExpression>,
): Array<TemplatePart> {
	let parts: Array<TemplatePart> = [];
	let queue: Array<TemplatePart> = [...expressions];

	while (true) {
		let node = queue.shift();
		if (!node) {
			break;
		}

		if (node.type === "JSTemplateLiteral") {
			queue = [...zipTemplateLiteralParts(node), ...queue];
		} else {
			parts.push(node);
		}
	}

	return parts;
}

// 'str' + 'str' + expr -> 'strstr' + expr
function combineTemplateParts(expressions: Array<TemplatePart>) {
	let reducedExpressions: Array<AnyJSExpression> = [];
	let index = 0;

	while (index < expressions.length) {
		let current = expressions[index];

		if (
			current.type === "JSStringLiteral" ||
			current.type === "JSTemplateElement"
		) {
			let strings: Array<StaticString> = [current];

			while (index + 1 < expressions.length) {
				let next = expressions[index + 1];
				if (next.type === "JSStringLiteral" || next.type === "JSTemplateElement") {
					strings.push(next);
					index++;
				} else {
					break;
				}
			}

			if (strings.length === 1 && current.type === "JSStringLiteral") {
				reducedExpressions.push(current);
			} else {
				reducedExpressions.push(
					jsStringLiteral.create({
						value: strings.map((string) => {
							if (string.type === "JSTemplateElement") {
								return string.raw;
							} else {
								return string.value;
							}
						}).join(""),
					}),
				);
			}
		} else {
			reducedExpressions.push(current);
		}

		index++;
	}

	return reducedExpressions;
}

function createEmptyQuasis(isTail: boolean = false) {
	return jsTemplateElement.create({
		cooked: "",
		raw: "",
		tail: isTail,
	});
}

// 'str' + expr + 'str' -> `str${expr}str`
function convertTemplatePartsToTemplateLiteral(
	nodes: Array<TemplatePart>,
): JSStringLiteral | JSTemplateLiteral {
	let templateExpressions: Array<AnyJSExpression> = [];
	let templateQuasis: Array<JSTemplateElement> = [];

	for (let index = 0; index < nodes.length; index++) {
		let node = nodes[index];
		let isTail = index === nodes.length - 1;
		let isHead = index === 0;

		if (node.type === "JSTemplateElement") {
			templateQuasis.push(node);
		} else if (node.type === "JSStringLiteral") {
			templateQuasis.push(
				jsTemplateElement.create({
					cooked: node.value,
					raw: node.value,
					tail: isTail,
				}),
			);
		} else {
			templateExpressions.push(node);

			let next = nodes[index + 1];
			let isNextQuasis =
				next?.type === "JSStringLiteral" || next?.type === "JSTemplateElement";

			if (isTail || isHead || !isNextQuasis) {
				templateQuasis.push(createEmptyQuasis(isTail));
			}
		}
	}

	return jsTemplateLiteral.create({
		expressions: templateExpressions,
		quasis: templateQuasis,
	});
}

// Only convert to a template literal when at least one of the expressions isn't a string
// Concatenating together only strings is allowed as it's sometimes used to break up long lines
// Ignore:
// str + str
// str + str + str
// Replace:
// str + expr
// str + expr + str
function shouldReplace(expressions: Array<AnyJSExpression>): boolean {
	for (let expression of expressions) {
		if (expression.type !== "JSStringLiteral") {
			return true;
		}
	}

	return false;
}

export default createVisitor({
	name: "js/useTemplate",
	enter(path) {
		const {node} = path;

		if (isUnnecessaryStringConcatExpression(node)) {
			const expressions = collectBinaryAddExpressionExpressions(node);

			if (shouldReplace(expressions)) {
				const templateParts = flattenExpressionsToTemplateParts(expressions);
				const combinedParts = combineTemplateParts(templateParts);
				const template = convertTemplatePartsToTemplateLiteral(combinedParts);

				return path.addFixableDiagnostic(
					{
						fixed: signals.replace(template),
					},
					descriptions.LINT.JS_USE_TEMPLATE,
				);
			}
		}

		return signals.retain;
	},
});
