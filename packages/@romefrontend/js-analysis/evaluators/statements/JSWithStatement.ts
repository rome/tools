/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {AnyNode, JSWithStatement, jsWithStatement} from "@romefrontend/ast";

export default function JSWithStatement(node: AnyNode, scope: Scope) {
	node = jsWithStatement.assert(node);
	scope;
	throw new Error("unimplemented");
}
