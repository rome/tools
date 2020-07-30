/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {AnyNode, JSMemberExpression, jsMemberExpression} from "@internal/ast";
import StringLiteralT from "../../types/StringLiteralT";
import GetPropT from "../../types/GetPropT";

export default function JSMemberExpression(node: AnyNode, scope: Scope) {
	node = jsMemberExpression.assert(node);
	if (node.property.type === "JSComputedMemberProperty") {
		throw new Error("Computed properties not supportd yet");
	}

	if (node.property.value.type === "JSPrivateName") {
		throw new Error("PrivateName in static member not supported yet");
	}

	const prop = new StringLiteralT(
		scope,
		node.property.value,
		node.property.value.name,
	);
	return new GetPropT(scope, node, scope.evaluate(node.object), prop);
}
