/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {AnyNode, jsUnaryExpression} from "@internal/ast";
import BooleanT from "../../types/BooleanT";
import NumericT from "../../types/NumericT";
import VoidT from "../../types/VoidT";
import TypeofT from "../../types/TypeofT";

export default function JSUnaryExpression(node: AnyNode, scope: Scope) {
	node = jsUnaryExpression.assert(node);
	const argType = scope.evaluate(node.argument);

	switch (node.operator) {
		case // booleans
		"delete":
		case "!":
			return new BooleanT(scope, node);

		// numbers
		case "+":
		case "-":
		case "~":
			return new NumericT(scope, node);

		// strings
		case "typeof":
			return new TypeofT(scope, node, argType);

		// void
		case "void":
			return new VoidT(scope, node);

		// empty!
		case "throw":
			break;
	}

	return undefined;
}
