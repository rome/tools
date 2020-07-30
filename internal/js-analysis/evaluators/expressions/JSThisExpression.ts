/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope, ThisScope} from "../../scopes";
import {AnyNode, jsThisExpression} from "@internal/ast";
import OpenT from "../../types/OpenT";

export default function JSThisExpression(node: AnyNode, scope: Scope) {
	node = jsThisExpression.assert(node);
	const thisScope = scope.find(ThisScope);
	if (thisScope === undefined) {
		// TODO complain
		return undefined;
	} else {
		const type = new OpenT(scope, node);
		type.shouldMatch(thisScope.context);
		return type;
	}
}
