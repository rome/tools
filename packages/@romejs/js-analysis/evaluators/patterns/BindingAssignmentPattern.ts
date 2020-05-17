/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {
	AnyNode,
	BindingAssignmentPattern,
	bindingAssignmentPattern,
} from "@romejs/js-ast";

export default function BindingAssignmentPattern(node: AnyNode, scope: Scope) {
	node = bindingAssignmentPattern.assert(node);
	scope;
	throw new Error("unimplemented");
}
