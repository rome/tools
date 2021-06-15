/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {createLintVisitor, signals} from "@internal/compiler";
import {
	JSCallExpression,
	JSOptionalCallExpression,
	JSXElement,
} from "@internal/ast";
import {descriptions} from "@internal/diagnostics";
import {doesNodeMatchPattern} from "@internal/js-ast-utils";

function containsKeyAttr(node: JSXElement): boolean {
	const ATTR_NAME = "key";
	return !!node.attributes.find((attr) =>
		attr.type === "JSXAttribute" && attr.name.name === ATTR_NAME
	);
}

function getMapCallback(node: JSCallExpression | JSOptionalCallExpression) {
	if (
		doesNodeMatchPattern(node.callee, "React.Children.map") ||
		doesNodeMatchPattern(node.callee, "Children.map")
	) {
		return node.arguments[1];
	}

	if (
		node.callee.type === "JSMemberExpression" &&
		node.callee.property.value.type === "JSIdentifier" &&
		node.callee.property.value.name === "map"
	) {
		return node.arguments[0];
	}

	return undefined;
}

export default createLintVisitor({
	name: "react/useKey",
	enter(path) {
		const {node, context} = path;

		// JSXElement in array literal
		if (
			node.type === "JSXElement" &&
			!containsKeyAttr(node) &&
			path.parentPath.node.type === "JSArrayExpression"
		) {
			context.addNodeDiagnostic(node, descriptions.LINT.REACT_USE_KEY("array"));
		}

		const fn =
			(node.type === "JSCallExpression" ||
			node.type === "JSOptionalCallExpression") &&
			getMapCallback(node);

		// Array.prototype.map
		if (fn) {
			// Short hand arrow function
			if (
				fn.type === "JSArrowFunctionExpression" &&
				fn.body.type === "JSXElement" &&
				!containsKeyAttr(fn.body)
			) {
				context.addNodeDiagnostic(
					fn.body,
					descriptions.LINT.REACT_USE_KEY("iterator"),
				);
			}

			// Function or arrow function with block statement
			if (
				fn &&
				(fn.type === "JSFunctionExpression" ||
				fn.type === "JSArrowFunctionExpression") &&
				fn.body.type === "JSBlockStatement"
			) {
				for (const statement of fn.body.body) {
					if (
						statement.type === "JSReturnStatement" &&
						statement.argument?.type === "JSXElement" &&
						!containsKeyAttr(statement.argument)
					) {
						context.addNodeDiagnostic(
							statement.argument,
							descriptions.LINT.REACT_USE_KEY("iterator"),
						);
					}
				}
			}
		}

		return signals.retain;
	},
});
