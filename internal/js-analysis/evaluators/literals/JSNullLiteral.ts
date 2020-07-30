/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {AnyNode, JSNullLiteral, jsNullLiteral} from "@internal/ast";
import NullT from "../../types/NullT";

export default function JSNullLiteral(node: AnyNode, scope: Scope) {
	node = node = jsNullLiteral.assert(node);
	return new NullT(scope, node);
}
