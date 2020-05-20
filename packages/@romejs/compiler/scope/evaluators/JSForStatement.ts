/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Scope from "../Scope";
import {AnyNode, JSForStatement} from "@romejs/ast";

export default {
	creator: true,
	build(node: JSForStatement, parent: AnyNode, scope: Scope) {
		const newScope = scope.fork("loop", node);
		newScope.evaluate(node.init, node);
		return newScope;
	},
};
