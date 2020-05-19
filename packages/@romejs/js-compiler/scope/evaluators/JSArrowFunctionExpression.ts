/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Scope from "../Scope";
import {AnyNode, JSArrowFunctionExpression} from "@romejs/ast";

export default {
	creator: true,
	build(node: JSArrowFunctionExpression, parent: AnyNode, scope: Scope) {
		return scope.evaluate(node.head, node, true);
	},
};
