/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {AnyNode, JSMockParent, jsMockParent} from "@romejs/ast";

export default function JSMockParent(node: AnyNode, scope: Scope) {
	node = jsMockParent.assert(node);
	scope;
	throw new Error("unimplemented");
}
