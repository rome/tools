/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {AnyNode, JSArrayExpression, jsArrayExpression} from "@internal/ast";
import InstanceT from "../../types/InstanceT";
import OpenT from "../../types/OpenT";

export default function JSArrayExpression(node: AnyNode, scope: Scope) {
	node = jsArrayExpression.assert(node);
	const elems = [];

	for (const expr of node.elements) {
		if (expr === undefined) {
			// TODO array hole, add undefined here
		} else {
			elems.push(scope.evaluate(expr));
		}
	}

	let value;
	if (elems.length === 0) {
		value = new OpenT(scope, node);
	} else {
		value = scope.createUnion(elems, node);
	}
	return new InstanceT(scope, node, scope.intrinsics.Array, [value]);
}
