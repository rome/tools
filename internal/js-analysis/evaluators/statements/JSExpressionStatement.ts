/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {
	AnyNode,
	JSExpressionStatement,
	jsExpressionStatement,
} from "@internal/ast";

export default function JSExpressionStatement(node: AnyNode, scope: Scope) {
	node = jsExpressionStatement.assert(node);

	return scope.evaluate(node.expression);
}
