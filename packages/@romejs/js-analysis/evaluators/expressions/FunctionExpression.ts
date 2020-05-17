/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {AnyNode, FunctionExpression, functionExpression} from "@romejs/js-ast";
import executeFunction from "../../utils/executeFunction";

export default function FunctionExpression(node: AnyNode, scope: Scope) {
	node = functionExpression.assert(node);
	return executeFunction(node, scope, true);
}
