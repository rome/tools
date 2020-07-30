/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {
	AnyNode,
	JSClassPrivateMethod,
	jsClassPrivateMethod,
} from "@internal/ast";

export default function JSClassPrivateMethod(node: AnyNode, scope: Scope) {
	node = jsClassPrivateMethod.assert(node);
	scope;
	throw new Error("unimplemented");
}
