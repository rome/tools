/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {AnyNode, JSNumericLiteral, jsNumericLiteral} from "@internal/ast";
import NumericLiteralT from "../../types/NumericLiteralT";

export default function JSNumericLiteral(node: AnyNode, scope: Scope) {
	node = jsNumericLiteral.assert(node);
	return new NumericLiteralT(scope, node, node.value);
}
