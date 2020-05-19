/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {AnyNode, TSFunctionType, tsFunctionType} from "@romejs/ast";

export default function TSFunctionType(node: AnyNode, scope: Scope) {
	node = tsFunctionType.assert(node);
	scope;
	throw new Error("unimplemented");
}
