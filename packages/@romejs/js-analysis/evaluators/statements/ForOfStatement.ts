/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {AnyNode, ForOfStatement, forOfStatement} from "@romejs/js-ast";

export default function ForOfStatement(node: AnyNode, scope: Scope) {
	node = forOfStatement.assert(node);
	scope;
	throw new Error("unimplemented");
}
