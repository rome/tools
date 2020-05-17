/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {AnyNode} from "@romejs/js-ast";
import IfStatement from "../statements/IfStatement";

export default function ConditionalExpression(node: AnyNode, scope: Scope) {
	return IfStatement(node, scope);
}
