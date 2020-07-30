/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {AnyNode} from "@internal/ast";
import JSIfStatement from "../statements/JSIfStatement";

export default function JSConditionalExpression(node: AnyNode, scope: Scope) {
	return JSIfStatement(node, scope);
}
