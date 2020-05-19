/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Scope from "../Scope";
import {AnyNode, JSClassDeclaration, JSClassExpression} from "@romejs/ast";

export default {
	creator: true,
	build(
		node: JSClassExpression | JSClassDeclaration,
		parent: AnyNode,
		scope: Scope,
	) {
		const newScope = scope.fork("class", node);
		newScope.evaluate(node.meta.typeParameters);
		return newScope;
	},
};
