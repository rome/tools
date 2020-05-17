/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {AnyNode, StringLiteral, stringLiteral} from "@romejs/js-ast";
import StringLiteralT from "../../types/StringLiteralT";

export default function StringLiteral(node: AnyNode, scope: Scope) {
	node = stringLiteral.assert(node);
	return new StringLiteralT(scope, node, node.value);
}
