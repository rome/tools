/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {AnyNode, JSBooleanLiteral, jsBooleanLiteral} from "@internal/ast";
import BooleanLiteralT from "../../types/BooleanLiteralT";

export default function JSBooleanLiteral(node: AnyNode, scope: Scope) {
	node = jsBooleanLiteral.assert(node);
	return new BooleanLiteralT(scope, node, node.value);
}
