/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {AnyNode, JSFunctionHead, jsFunctionHead} from "@romejs/ast";

export default function JSFunctionHead(node: AnyNode, scope: Scope) {
	node = jsFunctionHead.assert(node);
	scope;
	throw new Error("unimplemented");
}
