/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {AnyNode, JSRoot, jsRoot} from "@internal/ast";
import JSBlockStatement from "../statements/JSBlockStatement";

export default function JSRoot(node: AnyNode, scope: Scope) {
	node = jsRoot.assert(node);
	JSBlockStatement(node, scope);
}
