/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from "@romefrontend/compiler";
import {AnyNode, JSCallExpression, JSXElement} from "@romefrontend/ast";
import {descriptions} from "@romefrontend/diagnostics";
import {doesNodeMatchPattern} from "@romefrontend/js-ast-utils";

function containsKeyAttr(node: JSXElement): boolean {
	const ATTR_NAME = "key";
	return !!node.attributes.find((attr) =>
		attr.type === "JSXAttribute" && attr.name.name === ATTR_NAME
	);
}

function getMapCallback(node: JSCallExpression) {
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

export default {
	name: "react/jsxKey",
	enter(path: Path): AnyNode {
		const {node, context} = path;

		// JSXElement in array literal
		if (
			node.type === "JSXElement" &&
			!containsKeyAttr(node) &&
			path.parentPath.node.type === "JSArrayExpression"
		) {
			context.addNodeDiagnostic(node, descriptions.LINT.REACT_JSX_KEY("array"));
		}

		const fn = node.type === "JSCallExpression" && getMapCallback(node);

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
					descriptions.LINT.REACT_JSX_KEY("iterator"),
				);
			}

			// Function or arrow function with block statement
			if (
				fn &&
				(fn.type === "JSFunctionExpression" ||
				fn.type === "JSArrowFunctionExpression") &&
				fn.body.type === "JSBlockStatement"
			) {
				fn.body.body.forEach((statement) => {
					if (
						statement.type === "JSReturnStatement" &&
						statement.argument?.type === "JSXElement" &&
						!containsKeyAttr(statement.argument)
					) {
						context.addNodeDiagnostic(
							statement.argument,
							descriptions.LINT.REACT_JSX_KEY("iterator"),
						);
					}
				});
			}
		}

		return node;
	},
};
