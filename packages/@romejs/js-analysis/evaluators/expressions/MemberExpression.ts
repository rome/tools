/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {AnyNode, MemberExpression, memberExpression} from "@romejs/js-ast";
import StringLiteralT from "../../types/StringLiteralT";
import GetPropT from "../../types/GetPropT";

export default function MemberExpression(node: AnyNode, scope: Scope) {
	node = memberExpression.assert(node);
	if (node.property.type === "ComputedMemberProperty") {
		throw new Error("Computed properties not supportd yet");
	}

	if (node.property.value.type === "PrivateName") {
		throw new Error("PrivateName in static member not supported yet");
	}

	const prop = new StringLiteralT(
		scope,
		node.property.value,
		node.property.value.name,
	);
	return new GetPropT(scope, node, scope.evaluate(node.object), prop);
}
