/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {AnyNode, JSClassDeclaration, jsClassDeclaration} from "@internal/ast";
import JSClassExpression from "./JSClassExpression";

export default function JSClassDeclaration(node: AnyNode, scope: Scope) {
	node = jsClassDeclaration.assert(node);
	const type = JSClassExpression(node, scope);
	if (node.id) {
		scope.addBinding(node.id.name, type);
	}
	return type;
}
