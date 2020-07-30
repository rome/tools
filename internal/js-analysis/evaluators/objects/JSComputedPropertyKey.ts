/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {
	AnyNode,
	JSComputedPropertyKey,
	jsComputedPropertyKey,
} from "@internal/ast";

export default function JSComputedPropertyKey(node: AnyNode, scope: Scope) {
	node = jsComputedPropertyKey.assert(node);
	scope;
	throw new Error("unimplemented");
}
