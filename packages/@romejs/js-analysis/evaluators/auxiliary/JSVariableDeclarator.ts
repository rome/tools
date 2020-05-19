/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {AnyNode, JSVariableDeclarator, jsVariableDeclarator} from "@romejs/ast";

export default function JSVariableDeclarator(node: AnyNode, scope: Scope) {
	node = jsVariableDeclarator.assert(node);
	scope;
	throw new Error("unimplemented");
}
