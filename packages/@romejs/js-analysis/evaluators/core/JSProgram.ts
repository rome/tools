/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {AnyNode, JSRoot, JSRoot} from "@romejs/ast";
import JSBlockStatement from "../statements/JSBlockStatement";

export default function JSRoot(node: AnyNode, scope: Scope) {
	node = JSRoot.assert(node);
	JSBlockStatement(node, scope);
}
