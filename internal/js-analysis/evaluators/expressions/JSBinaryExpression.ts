/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {AnyNode, jsBinaryExpression} from "@internal/ast";
import Evaluator from "../../Evaluator";
import NumericT from "../../types/NumericT";
import ExhaustiveT from "../../types/ExhaustiveT";
import RefineTypeofT from "../../types/RefineTypeofT";
import BinaryOpT from "../../types/BinaryOpT";

function maybeRefine(
	node: AnyNode,
	left: AnyNode,
	right: AnyNode,
	scope: Scope,
): boolean {
	const evaluator: Evaluator = scope.evaluator;

	if (left.type === "JSIdentifier") {
		scope.addBinding(left.name, evaluator.getTypeFromEvaluatedNode(right));
		return true;
	}

	if (
		left.type === "JSUnaryExpression" &&
		left.operator === "typeof" &&
		left.argument.type === "JSReferenceIdentifier"
	) {
		const name = left.argument.name;
		const binding = scope.getBinding(name);
		if (binding !== undefined) {
			const type = new RefineTypeofT(
				scope,
				node,
				evaluator.getTypeFromEvaluatedNode(right),
				binding,
			);
			scope.addBinding(name, type);
			return true;
		}
	}

	return false;
}

export default function JSBinaryExpression(node: AnyNode, scope: Scope) {
	node = jsBinaryExpression.assert(node);

	const left = scope.evaluate(node.left);
	const right = scope.evaluate(node.right);

	// Enforce that the left and right sides of these operators are numbers
	switch (node.operator) {
		case "<<":
		case ">>":
		case ">>>":
		case "-":
		case "*":
		case "/":
		case "%":
		case "**":
		case "|":
		case "^":
		case "&":
		case "<":
		case "<=":
		case ">":
		case ">=": {
			const num = new NumericT(scope, undefined);
			new ExhaustiveT(scope, node, left, num);
			new ExhaustiveT(scope, node, right, num);
			break;
		}
	}

	// Refinements
	let refinedScope = scope;
	if (node.operator === "===") {
		refinedScope = scope.refine();
		maybeRefine(node, node.left, node.right, refinedScope) ||
		maybeRefine(node, node.right, node.left, refinedScope);
	}

	return new BinaryOpT(refinedScope, node, left, node.operator, right);
}
