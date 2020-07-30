/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {
	AnyNode,
	JSComputedMemberProperty,
	jsComputedMemberProperty,
} from "@internal/ast";

export default function JSComputedMemberProperty(node: AnyNode, scope: Scope) {
	node = jsComputedMemberProperty.assert(node);
	scope;
	throw new Error("unimplemented");
}
