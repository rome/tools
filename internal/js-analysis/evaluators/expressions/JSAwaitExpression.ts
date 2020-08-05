/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {AnyNode, jsAwaitExpression} from "@internal/ast";

export default function JSAwaitExpression(node: AnyNode, scope: Scope) {
	node = jsAwaitExpression.assert(node);
	scope;
	throw new Error("unimplemented");
}
