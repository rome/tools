/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {
	AnyNode,
	JSInterpreterDirective,
	jsInterpreterDirective,
} from "@romejs/ast";

export default function JSInterpreterDirective(node: AnyNode, scope: Scope) {
	node = jsInterpreterDirective.assert(node);
	scope;
	throw new Error("unimplemented");
}
