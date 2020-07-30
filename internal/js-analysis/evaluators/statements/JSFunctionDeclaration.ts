/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {
	AnyNode,
	JSFunctionDeclaration,
	jsFunctionDeclaration,
} from "@internal/ast";
import executeFunction from "../../utils/executeFunction";

export default function JSFunctionDeclaration(node: AnyNode, scope: Scope) {
	node = jsFunctionDeclaration.assert(node);

	const func = executeFunction(node, scope, false);
	if (node.id !== undefined) {
		scope.addBinding(node.id.name, func);
	}
	return func;
}
