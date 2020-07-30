/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {
	AnyNode,
	JSSequenceExpression,
	jsSequenceExpression,
} from "@internal/ast";

export default function JSSequenceExpression(node: AnyNode, scope: Scope) {
	node = jsSequenceExpression.assert(node);
	scope;
	throw new Error("unimplemented");
}
