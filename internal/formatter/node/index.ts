/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from "@internal/ast";
import {SourceLocation, comparePositions} from "@internal/parser-core";
import parentheses from "./parentheses";

function isOrHasCallExpression(node: AnyNode): boolean {
	if (node.type === "JSCallExpression") {
		return true;
	}

	if (node.type === "JSComputedMemberProperty") {
		return isOrHasCallExpression(node.value);
	}

	if (node.type === "JSMemberExpression") {
		return (
			isOrHasCallExpression(node.object) || isOrHasCallExpression(node.property)
		);
	}

	return false;
}

function orderLoc(
	a: SourceLocation,
	b: SourceLocation,
): [SourceLocation, SourceLocation] {
	if (comparePositions(a.end, b.start) === -1) {
		return [a, b];
	} else {
		return [b, a];
	}
}

export function getLinesBetween(aNode: AnyNode, bNode: AnyNode): number {
	if (aNode.loc && bNode.loc) {
		const [a, b] = orderLoc(aNode.loc, bNode.loc);
		return b.start.line.valueOf() - a.end.line.valueOf();
	} else {
		return 0;
	}
}

export function needsParens(
	node: AnyNode,
	parent: undefined | AnyNode,
	printStack: AnyNode[],
): boolean {
	if (!parent) {
		return false;
	}

	if (parent.type === "JSNewExpression" && parent.callee === node) {
		if (isOrHasCallExpression(node)) {
			return true;
		}
	}

	const fn = parentheses.get(node.type);
	return fn ? fn(node, parent, printStack) : false;
}
