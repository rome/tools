/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {AnyNode, jsAssignmentExpression} from "@internal/ast";
import SideEffectT from "../../types/SideEffectT";

export default function JSAssignmentExpression(node: AnyNode, scope: Scope) {
	node = jsAssignmentExpression.assert(node);

	const {left, right, operator} = node;

	if (operator === "=") {
		const rightType = scope.evaluate(right);
		const leftType = scope.evaluate(left);
		leftType.shouldMatch(rightType);
		return new SideEffectT(scope, node, rightType);
	} else {
		// TODO!
		return undefined;
	}
}
