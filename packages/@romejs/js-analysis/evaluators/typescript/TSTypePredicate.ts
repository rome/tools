/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {AnyNode, TSTypePredicate, tsTypePredicate} from "@romejs/ast";

export default function TSTypePredicate(node: AnyNode, scope: Scope) {
	node = tsTypePredicate.assert(node);
	scope;
	throw new Error("unimplemented");
}
