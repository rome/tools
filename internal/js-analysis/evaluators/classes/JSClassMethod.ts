/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ClassScope, Scope} from "../../scopes";
import {AnyNode, JSClassMethod, jsClassMethod} from "@internal/ast";
import ObjPropT from "../../types/ObjPropT";
import executeFunction from "../../utils/executeFunction";

export default function JSClassMethod(node: AnyNode, scope: Scope) {
	node = jsClassMethod.assert(node);
	if (node.key.type === "JSComputedPropertyKey") {
		// TODO
		return undefined;
	}

	const classScope = scope.find(ClassScope);
	const thisContext =
		node.meta.static === true
			? classScope.meta.static
			: classScope.meta.instance;
	const func = executeFunction(node, scope, false, thisContext);

	if (node.key.value.type !== "JSIdentifier") {
		throw new Error("Expected only an jsIdentifier key");
	}
	return new ObjPropT(scope, node, node.key.value.name, func);
}
